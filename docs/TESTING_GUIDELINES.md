# Rust 테스트 가이드라인

이 문서는 Tetris 프로젝트의 테스트 규칙과 모범 사례를 정리한 문서입니다. 우리는 테스트의 가독성과 유지보수성을 높이고, 시스템의 예상 동작을 명확히 문서화하기 위해 **Given-When-Then** (또는 Arrange-Act-Assert) 패턴을 따릅니다.

## 1. 테스트 구조: Given-When-Then

모든 테스트 함수는 세 가지 명확한 섹션으로 구성되어야 합니다. 주석을 사용하여 이 섹션들을 명시적으로 구분해주세요.

*   **Given (Arrange/준비)**: 초기 상태를 설정하고, 객체를 초기화하며, 입력을 준비합니다.
*   **When (Act/실행)**: 테스트하려는 특정 동작이나 함수를 실행합니다.
*   **Then (Assert/검증)**: 결과를 확인하고, 상태 변화를 체크하며, 결과가 예상과 일치하는지 검증합니다.

### 예시

```rust
#[test]
fn should_calculate_total_score_correctly() {
    // Given: 특정 게임 상태를 가진 사용자
    let mut game = Game::new();
    let initial_score = game.score;

    // When: 사용자가 라인을 클리어했을 때
    game.clear_lines(1);

    // Then: 점수가 예상한 만큼 증가해야 한다
    assert_eq!(game.score, initial_score + 100);
}
```

## 2. 네이밍 컨벤션 (명명 규칙)

테스트 함수 이름은 **무엇을** 테스트하고 **어떤 결과**가 예상되는지 설명하는 서술형 문장이어야 합니다. `test_move`나 `test_add` 같은 포괄적인 이름은 피해주세요.

*   **Bad**: `fn test_move_left() {}`
*   **Good**: `fn move_left_decrements_piece_x_coordinate() {}`
*   **Good**: `fn given_empty_space_when_move_left_then_piece_moves() {}`

## 3. 통합 테스트 vs 단위 테스트

*   **단위 테스트 (Unit Tests)**: 소스 파일(예: `src/game/mod.rs`) 내부의 `#[cfg(test)] mod tests` 블록 안에 위치합니다. 비공개(private) 필드와 메서드에 접근할 수 있습니다.
*   **통합 테스트 (Integration Tests)**: `tests/` 디렉토리에 위치합니다. 이 테스트들은 해당 크레이트를 외부 라이브러리처럼 취급하며 오직 공개(public) API에만 접근할 수 있습니다.

## 4. 테스트 셋업: 빌더 패턴 (Builder Pattern)

특정한 테스트 시나리오("Given" 상태)를 만들기 위해 `App`과 `Board`에서 제공하는 빌더 패턴 메서드를 사용하세요. 이를 통해 내부 필드를 수동으로 수정하지 않고도 테스트 셋업을 명시적이고 읽기 쉽게 만들 수 있습니다.

*   `App::new().with_board(...)`: 커스텀 보드 상태를 주입합니다.
*   `App::new().with_current_piece(...)`: 특정 조각 타입을 주입합니다.
*   `board.set_cell(x, y, color)`: 보드에 블록을 배치하는 헬퍼 메서드입니다.

### 예시
```rust
let app = App::new()
    .with_board(custom_board)
    .with_current_piece(Piece::new(TetrominoType::I));
```

## 5. 모범 사례 (Best Practices)

*   **테스트당 하나의 개념 (One Concept Per Test)**: 하나의 테스트 함수에서는 하나의 논리적 시나리오만 검증하세요. 관련 없는 체크들을 섞지 마세요 (예: 왼쪽 벽 충돌과 오른쪽 벽 충돌을 한 함수에서 모두 체크하는 것 금지).
*   **설명적인 검증문 (Descriptive Assertions)**: 검증문(assertion)에 커스텀 에러 메시지를 제공하세요.
    *   `assert!(condition, "Error message explaining why it failed");`
*   **구현이 아닌 동작을 테스트하라**: 코드가 '어떻게' 동작하는지(내부 상태)보다 '무엇을' 하는지(공개 API)에 집중하세요.
