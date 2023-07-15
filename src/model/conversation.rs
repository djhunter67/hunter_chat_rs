// Purpose: Conversation struct and methods

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Conversation {
    messages: Vec<Message>,
}

impl Conversation {
    pub fn new() -> Conversation {
        Conversation {
            messages: Vec::new(),
        }
    }

    // pub fn add_message(&mut self, message: Message) {
    // 	self.messages.push(message);
    // }

    // pub fn get_messages(&self) -> &Vec<Message> {
    // 	&self.messages
    // }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Message {
    id: i32,
    text: String,
}
