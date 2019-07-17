mod combinators;
mod outer_message;
mod protos;

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
    pub fn begin_parse(&self, input: &[u8]) {
        let input = match combinators::take_source2_signature(input) {
            Ok((inp, _)) => inp,
            _ => panic!("Couldn't verify Source 2 signature in the provided file."),
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
                Ok((remainder, _)) => remainder,
                _ => panic!("Unknown outer message."),
            };
        }
    }

    /// Signals the parser to stop parsing any further.
    pub fn stop_parse(&mut self) {
        self.is_stopping = true;
    }

    /// Signals the parser to stop parsing at a specific game tick.
    /// If `stop_tick` has already been passed, this will do nothing.
    pub fn stop_parse_at_tick(&mut self, stop_tick: u32) {
        self.stop_at_tick = Some(stop_tick);
    }

    /// Returns the last game tick that the parser processed.
    pub fn current_tick(&self) -> u32 {
        self.current_tick
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let data = include_bytes!("../assets/replay1.dem");
        let parser = Hyperstone::new();
        parser.begin_parse(data);
    }
}
