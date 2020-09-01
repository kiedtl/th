use lib::priority::*;

// a game message
// e.g. "the kobold strikes you with her lead mace!"
pub struct Message {
    pub text: String,
    pub priority: Priority,
}

impl Message {
    // TODO: msg!() macro?
    pub fn new(s: &str, p: Priority) -> Message {
        Message {
            text: s.to_string(),
            priority: p,
        }
    }
}
