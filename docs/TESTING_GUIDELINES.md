# Rust Testing Guidelines

This document outlines the testing conventions and best practices for the Tetris project. We follow the **Given-When-Then** (or Arrange-Act-Assert) pattern to ensure tests are readable, maintainable, and clearly document the expected behavior of the system.

## 1. Test Structure: Given-When-Then

Every test function should be structured into three distinct sections. Use comments to demarcate these sections explicitly.

*   **Given (Arrange)**: Set up the initial state, initialize objects, and prepare inputs.
*   **When (Act)**: Execute the specific action or function being tested.
*   **Then (Assert)**: Verify the results, check state changes, and ensure the outcome matches expectations.

### Example

```rust
#[test]
fn should_calculate_total_score_correctly() {
    // Given: A user with a specific game state
    let mut game = Game::new();
    let initial_score = game.score;

    // When: The user clears a line
    game.clear_lines(1);

    // Then: The score should increase by the expected amount
    assert_eq!(game.score, initial_score + 100);
}
```

## 2. Naming Conventions

Test function names should be descriptive sentences that explain **what** is being tested and **what the expected outcome** is. Avoid generic names like `test_move` or `test_add`.

*   **Bad**: `fn test_move_left() {}`
*   **Good**: `fn move_left_decrements_piece_x_coordinate() {}`
*   **Good**: `fn given_empty_space_when_move_left_then_piece_moves() {}`

## 3. Integration vs Unit Tests

*   **Unit Tests**: Located inside the source file (e.g., `src/game/mod.rs`) within a `#[cfg(test)] mod tests` block. These tests have access to private fields and methods.
*   **Integration Tests**: Located in the `tests/` directory. These tests treat the crate as an external library and only access public APIs (`pub`).

## 4. Best Practices

*   **One Assert Per Concept**: Try to verify one logical concept per test. It makes debugging easier when a test fails.
*   **Descriptive Assertions**: When possible, provide custom error messages in assertions.
    *   `assert!(condition, "Error message explaining why it failed");`
*   **Test Behavior, Not Implementation**: Focus on what the code does (public API), not how it does it (internal state), especially for integration tests.
