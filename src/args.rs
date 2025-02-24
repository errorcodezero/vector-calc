use clap::{ArgAction, Parser};

#[derive(Parser, Debug)]
pub enum Args {
    Split {
        #[clap(long, short, action=ArgAction::SetFalse)]
        radians: bool,
        vector: f32,
        angle: f32,
    },
    Combine {
        x: f32,
        y: f32,
    },
}
