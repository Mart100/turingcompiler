use super::prelude::*;

pub enum TuringAction {
    L,
    R,
    S,
}

pub struct TuringInstruction {
    pub state: String,
    pub read: TapeSymbols,
    pub write: TapeSymbols,
    pub action: TuringAction,
    pub next_state: String,
}
