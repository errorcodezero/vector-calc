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
    Add {
        #[clap(long, short, action=ArgAction::SetFalse)]
        radians: bool,
        vector1: f32,
        angle1: f32,
        vector2: f32,
        angle2: f32,
    },
    Subtract {
        #[clap(long, short, action=ArgAction::SetFalse)]
        radians: bool,
        vector1: f32,
        angle1: f32,
        vector2: f32,
        angle2: f32,
    },
}
