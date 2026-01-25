#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Key {
    Left,
    Right,
    Down,
    Rotate,
    QuickDrop,
    Hold,
    Quit,
    Enter,
}

#[derive(Debug)]
pub enum Event {
    Input(Key),
    Tick,
}
