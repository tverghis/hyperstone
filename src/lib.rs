mod combinators;
mod outer_message;
mod protos;

#[derive(Default)]
pub struct Hyperstone {
    current_tick: u32,
    stop_at_tick: Option<u32>,
}

impl Hyperstone {
    pub fn new() -> Hyperstone {
        Hyperstone {
            ..Default::default()
        }
    }

    pub fn begin_parse(&self, input: &[u8]) {
        let (input, _) = combinators::take_source2_signature(input).unwrap();
        let (input, _) = combinators::take_replay_size_info(input).unwrap();

        let mut input = input;

        loop {
            if let Some(tick) = self.stop_at_tick {
                if self.current_tick >= tick {
                    break;
                }
            }

            input = match combinators::take_outer_message(input) {
                Ok((remainder, _)) => remainder,
                _ => {
                    println!("Unknown outer message.");
                    std::process::exit(1);
                }
            };
        }
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
