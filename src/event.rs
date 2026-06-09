pub enum AppEvent {
    Tick,
    Key(crossterm::event::KeyEvent),
    DbLoaded(Vec<crate::app::Package>),
    AurLoaded(Vec<crate::app::Package>),
    Message(String, u64, bool), // msg, secs, keep
    ScriptFetched(String, String), // url, content
    ConsoleChunk(String),
    ConsoleFinished(bool),
    Resize,
    AurDetailsLoaded(crate::app::Package),
}
