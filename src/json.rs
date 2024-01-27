use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug)]
pub struct Text {
    pub text: String,
}

#[derive(Deserialize, Debug)]
pub struct SlackHook {
    pub garfield_random: String,
    pub ml_chat: String,
}
