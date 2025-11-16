pub Enum ApplicationState {
    Stopped,
    Paused,
    Running,
}

pub struct Context {
    pub state: ApplicationState,
    pub ticks: u64,
}
