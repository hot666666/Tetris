# Rust 소유권과 대여 규칙: 구조체 필드 동시 접근 문제

## 1. 문제 상황: "나 자신과의 싸움" (Borrow Checker Conflict)

테트리스의 이동 로직을 구현하던 중, 다음과 같은 상황에서 컴파일 에러가 발생했습니다.

### 상황
`App` 구조체는 `board`와 `current_piece`를 가지고 있습니다.
`move_left` 메서드(`&mut self`)에서 `current_piece`를 움직인 후, `check_collision`(`&self`)을 호출하여 충돌 여부를 검사하려 했습니다.

### 에러 코드 예시
```rust
fn move_left(&mut self) {
    if let Some(piece) = &mut self.current_piece { // [1] current_piece 가변 대여 (Mutable Borrow)
        piece.x -= 1;
        
        if self.check_collision() { // [2] self 전체 불변 대여 시도 (Immutable Borrow) -> 에러!
            piece.x += 1;
        }
    }
}
```

### 왜 안 되나?
Rust의 **대여 규칙(Borrowing Rules)** 때문입니다.
1.  `piece` 변수가 `self.current_piece`를 **가변 대여(`&mut`)** 중입니다.
2.  이 상태에서 `self.check_collision()`을 호출하면 `self` 전체를 **불변 대여(`&`)** 하려고 시도합니다.
3.  Rust는 **"가변 대여가 살아있는 동안에는, 불변 대여조차 할 수 없다"**는 철칙을 가지고 있습니다. (데이터 레이스 방지)

## 2. 해결 방안: "필요한 것만 빌리기" (Splitting Borrows)

Rust 컴파일러는 사실 꽤 똑똑해서, 구조체의 **서로 다른 필드**를 각각 빌리는 것은 허용합니다. 이를 **Splitting Borrows**라고 합니다.

### 핵심 아이디어
`check_collision` 메서드가 굳이 `&self` 전체를 다 가져갈 필요가 있을까요?
사실 충돌 검사에 필요한 건 `board`와 검사할 `piece` 뿐입니다.

### Refactoring: 시그니처 변경

**Before:**
`App`의 모든 정보를 다 보겠다. (`&self`)
```rust
fn check_collision(&self) -> bool { ... }
```

**After:**
`Board`와 `Piece`만 보여달라. (`&Board`, `&Piece`)
```rust
// Associated Function(static method)으로 만들거나, 
// self를 안 쓰는 메서드로 변경
fn check_collision(board: &Board, piece: &Piece) -> bool { ... }
```

### 적용 코드
이렇게 바꾸면 호출하는 쪽(`move_left`)에서 자연스럽게 대여가 분리됩니다.
```rust
fn move_left(&mut self) {
    if let Some(piece) = &mut self.current_piece {
        piece.x -= 1;
        
        // [Key Point]
        // self.board (불변 대여)와 piece (이미 가변 대여된 객체)를 넘깁니다.
        // 서로 다른 메모리 영역이므로 Rust가 흔쾌히 허용합니다.
        if check_collision(&self.board, piece) {
            piece.x += 1; // 되돌리기
        }
    }
}
```

## 3. 결론 및 교훈
*   **Struct 분해**: 메서드가 `&self`를 통째로 요구하면 사용하기 까다로워질 수 있습니다. 특히 `&mut self` 메서드 안에서 호출할 때 더욱 그렇습니다.
*   **함수의 순수성**: `check_collision`이 `App` 전체 상태에 의존하지 않고, 입력(`board`, `piece`)에만 의존하게 되어 더 함수형(Functional)이고 로직이 명확해졌습니다.
*   **성능**: 불필요한 `clone()` 없이 포인터(`&`)만 넘기므로 소유권 분쟁을 해결하면서도 성능 저하가 전혀 없습니다. (Zero-cost Abstraction)

이 패턴은 Rust 게임 개발이나 복잡한 구조체 로직 작성 시 필수적인 테크닉입니다.
