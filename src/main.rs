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
            let v = (x.powi(2) + y.powi(2)).sqrt();
            println!("{}", v);
        }
        Args::Add {
            vector1,
            angle1,
            vector2,
            angle2,
            radians,
        } => {
            let mut angle1 = angle1;
            let mut angle2 = angle2;
            if radians {
                angle1 /= 180.0;
                angle1 *= PI;

                angle2 /= 180.0;
                angle2 *= PI;
            }
            let x1 = angle1.cos() * vector1;
            let y1 = angle1.sin() * vector1;

            let x2 = angle2.cos() * vector2;
            let y2 = angle2.sin() * vector2;

            let x3 = x1 + x2;
            let y3 = y1 + y2;
            let v = (x3.powi(2) + y3.powi(2)).sqrt();

            println!("x: {x3}, y:{y3}, final vector: {v}")
        }
        Args::Subtract {
            vector1,
            angle1,
            vector2,
            angle2,
            radians,
        } => {
            let mut angle1 = angle1;
            let mut angle2 = angle2;
            if radians {
                angle1 /= 180.0;
                angle1 *= PI;

                angle2 /= 180.0;
                angle2 *= PI;
            }

            let x1 = angle1.cos() * vector1;
            let y1 = angle1.sin() * vector1;

            let x2 = angle2.cos() * vector2 * -1.0;
            let y2 = angle2.sin() * vector2 * -1.0;

            let x3 = x1 + x2;
            let y3 = y1 + y2;
            let v = (x3.powi(2) + y3.powi(2)).sqrt();

            println!("x: {x3}, y:{y3}, final vector: {v}")
        }
    }
}
