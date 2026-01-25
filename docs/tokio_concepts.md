# 테트리스 프로젝트의 Tokio 비동기 기술 활용

이 프로젝트는 Rust의 비동기 런타임인 **Tokio**를 활용하여 고성능이면서도 안전한 동시성 제어를 구현했습니다. 

## 1. mpsc (Multi-Producer, Single-Consumer) 채널

**개념**: 여러 태스크(Producer)에서 보낸 메시지를 하나의 수신자(Consumer)가 순차적으로 처리하는 데이터 파이프라인입니다.

*   **본 프로젝트에서의 역할**:
    - **중력 타이머 태스크**와 **사용자 입력 태스크**가 각자의 주기와 속도로 이벤트를 발생시켜 채널로 쏩니다.
    - 메인 루프는 채널에서 이벤트를 하나씩 꺼내어 처리함으로써, `App` 상태를 수정할 때 발생할 수 있는 데이터 경합(Race Condition)을 원천 차단합니다.
*   **코드 예시**:
    ```rust
    // 채널 생성 및 복사(Clone)를 통한 멀티 프로듀서 구성
    let (tx, mut rx) = mpsc::channel(100);
    let tx_gravity = tx.clone();
    let tx_input = tx.clone();

    // 메인 루프(Consumer)에서 순차 처리
    while let Some(event) = rx.recv().await {
        match event {
            Event::Input(key) => app.handle_input(key),
            Event::Tick => app.handle_input(Key::Down),
        }
    }
    ```

## 2. Graceful Shutdown (CancellationToken)

**개념**: 모든 백그라운드 태스크에 중단 신호를 한꺼번에 전달하여 안전하게 프로그램을 종료하는 메커니즘입니다.

*   **본 프로젝트에서의 역할**:
    - 사용자가 'Quit'을 하거나 게임이 끝날 때 `cancel_token.cancel()`을 호출합니다.
    - 백그라운드 태스크들은 `tokio::select!` 문 안에서 이 토큰의 신호를 대기하고 있다가, 신호가 오면 즉시 하던 일을 멈추고 루프를 빠져나옵니다.
*   **코드 예시**:
    ```rust
    let cancel_token = CancellationToken::new();

    tokio::spawn(async move {
        loop {
            tokio::select! {
                _ = cancel_token.cancelled() => break, // 종료 신호 수신 시 루프 탈출
                _ = interval.tick() => { /* 이벤트 송신 */ }
            }
        }
    });

    // 종료 시점
    cancel_token.cancel();
    ```

## 3. Backpressure (기압/역압 제어)

**개념**: 처리 속도보다 생성 속도가 빠를 때, 큐가 무한정 쌓이지 않도록 생산 속도를 늦추는 제어 방식입니다.

*   **본 프로젝트에서의 역할**:
    - `mpsc::channel(100)`과 같이 **버퍼의 크기를 제한(Bounded)**했습니다.
    - 만약 어떤 이유로 메인 루프가 100개 이상의 이벤트를 처리하지 못하고 밀리게 되면, 이벤트를 보내는 태스크들이 `send().await` 지점에서 잠시 대기하게 됩니다.
*   **코드 예시**:
    ```rust
    // 용량이 100인 Bounded Channel 생성
    let (tx, mut rx) = mpsc::channel(100);

    // 버퍼가 꽉 차면 아래 send().await는 비워질 때까지 비동기 대기(Suspended) 상태가 됨
    tx.send(Event::Tick).await.unwrap();
    ```