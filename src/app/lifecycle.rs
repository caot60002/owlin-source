#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ApplicationPhase {
    Starting,
    WaitingForGame,
    Injected,
    Terminating,
}
