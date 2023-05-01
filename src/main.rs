use traits_p::{Vector, Scalar};
use vector_derive::{vectorify, scalarify};

pub trait Energy {}
trait ForceTrait {}

#[vectorify]
struct Acceleration {}

#[vectorify]
struct Momentum {}

#[vectorify]
struct Velocity {}

#[scalarify]
struct KineticEnergy {}

impl Energy for KineticEnergy {}

trait PotentialEnergy{}

impl Energy for dyn PotentialEnergy {}

struct Object <'a> {
    mass: f64,
    volume: f64,
    forces: Vec<&'a dyn ForceTrait>,
    momentum: Momentum,
    velocity: Velocity
}

#[vectorify]
struct Force {}

impl ForceTrait for Force {}

impl  Force {
    fn new(acceleration: Acceleration, mass: f64) -> Self {
        let mag = acceleration.magnitude * mass;
        Force {magnitude: mag, angle: acceleration.angle}
    }
}

fn main() {
    let new_accel = Acceleration { magnitude: 10.0, angle: 10.0};
    let first = Force::new(new_accel, 100.0);

    println!("Magnitude: {} Newtons\nAngle: {} Degrees\nX-direction: {} Newtons\ny-direction: {} Newtons", first.get_magnitude(), first.get_angle(), first.get_x_component(), first.get_y_component())

}
