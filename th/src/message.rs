use crate::priority::*;

// a game message
// e.g. "the kobold strikes you with her lead mace!"
pub struct Message {
    text: String,
    priority: Priority,
}
