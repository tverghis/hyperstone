use crate::protos::demo::EDemoCommands;

pub(crate) trait DemoEvent {
    fn handle(&self);
}

impl DemoEvent for EDemoCommands {
    fn handle(&self) {
        println!("{:?}", &self);
    }
}
