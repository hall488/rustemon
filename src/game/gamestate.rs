#[derive(Debug, PartialEq)]
pub enum GameState {
    Paused,
    Running,
    Encounter,
    Party,
    Debug,
}
