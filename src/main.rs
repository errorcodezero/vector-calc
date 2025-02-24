use crate::args::Args;
use clap::Parser;
use std::f32::consts::PI;

pub mod args;

fn main() {
    let args = Args::parse();
    match args {
        Args::Split {
            vector,
            angle,
            radians,
        } => {
            let mut angle = angle;
            if radians {
                angle /= 180.0;
                angle *= PI;
            }

            let x = angle.cos() * vector;
            let y = angle.sin() * vector;

            println!("{x}, {y}")
        }
        Args::Combine { x, y } => {
            let v = (x.powf(2.0) + y.powf(2.0)).sqrt();
            println!("{}", v);
        }
    }
}
