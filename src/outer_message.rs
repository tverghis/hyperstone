#[derive(Debug)]
pub struct OuterMessage {
    tick: u32,
    message_type: u32,
    data: Vec<u8>,
}

impl OuterMessage {
    pub fn new(tick: u32, message_type: u32, data: Vec<u8>) -> OuterMessage {
        OuterMessage {
            tick,
            message_type,
            data,
        }
    }

    pub fn get_current_tick(&self) -> u32 {
        self.tick
    }

    pub fn get_message_type(&self) -> u32 {
        self.message_type
    }

    pub fn get_data(&self) -> &[u8] {
        &self.data
    }
}
