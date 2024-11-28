pub struct DataFrame {}

pub enum Opcode {
    Continuation,
    Text,
    Binary,
    NonControl1,
    NonControl2,
    NonControl3,
    NonControl4,
    NonControl5,
    Close,
    Ping,
    Pong,
    Control1,
    Control2,
    Control3,
    Control4,
    Control5,
}
