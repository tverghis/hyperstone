mod combinators;
mod demo_event;
mod errors;
mod outer_message;
mod protos; // Generated by `cargo build`

use demo_event::DemoEvent;
use errors::HyperstoneError;
use protobuf::{parse_from_bytes, Message};
use protos::demo::EDemoCommands::*;
use protos::demo::*;

/// Wrapper type around `Result` that specifically returns a [`HyperstoneError`].
///
/// [`HyperstoneError`]: enum.HyperstoneError.html
pub type HyperstoneParseResult<T> = Result<T, HyperstoneError>;

#[derive(Default)]
pub struct Hyperstone {
    current_tick: u32,
    stop_at_tick: Option<u32>,
    is_stopping: bool,
}

impl Hyperstone {
    pub fn new() -> Hyperstone {
        Hyperstone {
            ..Default::default()
        }
    }

    /// Starts parsing the replay buffer from the beginning.
    ///
    /// # Panics
    /// This function will panic in the following scenarios:
    /// 1. The passed-in buffer doesn't start with the expected Source 2 replay signature;
    /// 2. The buffer is shorter than expected;
    /// 3. The parser encounters an unidentifiable message.
    pub fn begin_parse(&self, input: &[u8]) -> HyperstoneParseResult<()> {
        let input = match combinators::take_source2_signature(input) {
            Ok((inp, _)) => inp,
            _ => return Err(HyperstoneError::UnverifiableBuffer),
        };

        let (input, _) = combinators::take_replay_size_info(input).unwrap();

        let mut input = input;

        loop {
            if self.is_stopping {
                break;
            }

            if let Some(tick) = self.stop_at_tick {
                if self.current_tick >= tick {
                    break;
                }
            }

            input = match combinators::take_outer_message(input) {
                Ok((remainder, message)) => {
                    if let Some(notifier) = notifier_for_demo_cmd(message.get_demo_cmd()?) {
                        notifier(message.get_data());
                    };
                    remainder
                }
                _ => return Err(HyperstoneError::UnknownOuterMessage),
            };
        }

        Ok(())
    }

    /// Signals the parser to stop parsing any further.
    pub fn stop_parse(&mut self) {
        self.is_stopping = true;
    }

    /// Signals the parser to stop parsing at a specific game tick.
    /// If `stop_tick` has already been crossed, this will do nothing.
    pub fn stop_parse_at_tick(&mut self, stop_tick: u32) {
        self.stop_at_tick = Some(stop_tick);
    }

    /// Returns the last game tick that the parser processed.
    pub fn current_tick(&self) -> u32 {
        self.current_tick
    }
}

// Returns the appropriately-typed `notify_on_demo_message` fn for a given demo command
fn notifier_for_demo_cmd(demo_cmd: EDemoCommands) -> Option<fn(&[u8])> {
    match demo_cmd {
        DEM_Stop => Some(notify_on_demo_message::<CDemoStop>),
        DEM_FileHeader => Some(notify_on_demo_message::<CDemoFileHeader>),
        DEM_FileInfo => Some(notify_on_demo_message::<CDemoFileInfo>),
        DEM_SyncTick => Some(notify_on_demo_message::<CDemoSyncTick>),
        DEM_SendTables => Some(notify_on_demo_message::<CDemoSendTables>),
        DEM_ClassInfo => Some(notify_on_demo_message::<CDemoClassInfo>),
        DEM_StringTables => Some(notify_on_demo_message::<CDemoStringTables>),
        DEM_Packet => Some(notify_on_demo_message::<CDemoPacket>),
        DEM_SignonPacket => Some(notify_on_demo_message::<CDemoPacket>), // <- still need to call notifiers for SignonPacket, not packet...
        DEM_ConsoleCmd => Some(notify_on_demo_message::<CDemoConsoleCmd>),
        DEM_CustomData => Some(notify_on_demo_message::<CDemoCustomData>),
        DEM_CustomDataCallbacks => Some(notify_on_demo_message::<CDemoCustomDataCallbacks>),
        DEM_UserCmd => Some(notify_on_demo_message::<CDemoUserCmd>),
        DEM_FullPacket => Some(notify_on_demo_message::<CDemoFullPacket>),
        DEM_SaveGame => Some(notify_on_demo_message::<CDemoSaveGame>),
        DEM_SpawnGroups => Some(notify_on_demo_message::<CDemoSpawnGroups>),
        _ => None,
    }
}

fn notify_on_demo_message<M: Message>(data: &[u8]) {
    let demo_message = parse_from_bytes::<M>(data);
    // notify listeners of M
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn bad_signature() {
        let data = b"thesearen'tthebytesyou'relookingfor";
        let parser = Hyperstone::new();
        assert_eq!(
            parser.begin_parse(data),
            Err(HyperstoneError::UnverifiableBuffer)
        );
    }

    #[test]
    fn test() {
        let data = include_bytes!("../assets/replay1.dem");
        let parser = Hyperstone::new();
        parser.begin_parse(data);
    }
}
