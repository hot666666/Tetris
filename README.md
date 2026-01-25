# Tetris TUI

Rust로 작성된 터미널용 간단한 테트리스 게임입니다.

![Tetris](image.png)

## 🚀 주요 기능

- **게임플레이**: 라인 클리어 및 점수 계산, 홀드 기능을 포함한 기본적인 테트리스를 구현했습니다.
- **TUI 인터페이스**: `ratatui`를 사용하여 반응성이 뛰어난 터미널 UI를 구축했습니다.
- **비동기 엔진**: `tokio`를 기반으로 효율적인 이벤트 루프를 구현했습니다.

## 🎮 게임 조작법

| 키 | 동작 |
| :--- | :--- |
| **← / →** | 왼쪽 / 오른쪽 이동 |
| **↑** | 회전 |
| **↓** | 소프트 드롭 (천천히 내리기) |
| **Space** | 하드 드롭 (즉시 내리기) |
| **H** | 블록 홀드 |
| **Q** | 종료 |

## 🛠️ 설치 및 실행

[Rust](https://www.rust-lang.org/tools/install)가 설치되어 있어야 합니다.

```bash
# 저장소 복제
git clone -b tetris2026-tui https://github.com/hot666666/Tetris.git
cd Tetris

# 게임 실행
cargo run --release
```

## 🏗️ 구성 소개

### 의존성
- **[ratatui](https://github.com/ratatui-org/ratatui)**: 터미널 UI 렌더링.
- **[crossterm](https://github.com/crossterm-rs/crossterm)**: 크로스 플랫폼 터미널 핸들링.
- **[tokio](https://tokio.rs/)**: 이벤트 루프를 위한 비동기 런타임.

### 게임 로직 흐름

핵심 게임 루프는 이동, 충돌 감지 및 블록 고정을 처리합니다.

```mermaid
graph TD
    Event{이벤트 타입}

    %% 1. Movement
    Event -- "이동" --> TryMove[사용자 입력 적용]
    TryMove --> CheckMove{충돌?}
    CheckMove -- 아니오 --> ApplyMove[위치 업데이트]
    CheckMove -- 예 --> RevertMove[취소 / 무시]

    %% 2. Gravity / Drop
    Event -- "(틱) 하강" --> TryDown[아래로 이동]
    TryDown --> CheckDown{충돌?}
    CheckDown -- 아니오 --> ApplyDown[Y축 업데이트]
    CheckDown -- 예 --> Lock[취소 및 블록 고정]
    
    subgraph "고정 시퀀스"
        Lock --> Clear[라인 클리어]
        Clear --> Spawn[다음 블록 생성]
        Spawn --> CheckOver{생성 위치 충돌?}
        CheckOver -- 예 --> GameOver[게임 오버]
        CheckOver -- 아니오 --> EnableHold[홀드 활성화]
    end

    %% 3. Hold
    Event -- "홀드" --> CheckUsed{홀드 사용됨?}
    CheckUsed -- 예 --> IgnoreHold[무시]
    CheckUsed -- 아니오 --> CheckEmpty{홀드 비어있음?}
    
    CheckEmpty -- 예 --> PushHold[현재 -> 홀드<br/>다음 블록 생성]
    CheckEmpty -- 아니오 --> SwapHold[현재 <-> 홀드 교체<br/>위치 초기화]
    
    PushHold --> MarkUsed[홀드 사용 표시]
    SwapHold --> MarkUsed
```

### 이벤트 파이프라인

입력 및 타이머 이벤트는 `tokio` 채널을 사용하여 동시에 처리됩니다.

```mermaid
graph TD
    subgraph "생성자 계층 (비동기 태스크)"
        GT["중력 태스크 (타이머)"]
        IT["입력 태스크 (Crossterm)"]
    end

    subgraph "시그널 계층"
        CT["CancellationToken"]
    end

    subgraph "통신"
        Chan["mpsc 채널"]
    end

    subgraph "소비자 계층 (메인 액터)"
        ML["메인 루프"]
        App["App 상태"]
    end

    CT -- "취소 브로드캐스트" --> GT
    CT -- "취소 브로드캐스트" --> IT
    
    GT -- "Event::Tick" --> Chan
    IT -- "Event::Input" --> Chan
    
    Chan -- "Receive" --> ML
    ML -- "업데이트" --> App
    App -- "그리기" --> UI["Ratatui 터미널"]
```

## 📜 라이선스

이 프로젝트는 MIT 라이선스에 따라 라이선스가 부여됩니다. 자세한 내용은 [LICENSE](LICENSE) 파일을 참조하십시오.