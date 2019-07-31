use crate::errors::HyperstoneError;
use crate::protos::demo::EDemoCommands;
use protobuf::ProtobufEnum;

/// `OuterMessage` represents a discrete message discovered by the parser.
#[derive(Debug)]
pub struct OuterMessage {
    tick: u32,
    message_type: u32,
    data: Vec<u8>,
}

impl OuterMessage {
    /// Creates a new `OuterMessage`.
    pub fn new(tick: u32, message_type: u32, data: Vec<u8>) -> OuterMessage {
        OuterMessage {
            tick,
            message_type,
            data,
        }
    }

    /// Tries to convert the internal message type into a known EDemoCommands variant.
    pub fn get_demo_cmd(&self) -> Result<EDemoCommands, HyperstoneError> {
        match EDemoCommands::from_i32(self.message_type as i32) {
            Some(demo_cmd) => Ok(demo_cmd),
            None => Err(HyperstoneError::UnknownDemoCommand),
        }
    }

    /// Returns the data associated with this message.
    #[allow(dead_code)]
    pub fn get_data(&self) -> &[u8] {
        &self.data
    }
}
