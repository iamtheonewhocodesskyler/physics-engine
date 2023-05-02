use std::fmt::Formatter;

use traits_p::{Vector, Scalar};
use vector_derive::{vectorify, scalarify};


trait Energy {}
trait ForceTrait {}

#[scalarify]
struct Mass {}

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
    velocity: Velocity,
    kinetic_energy: KineticEnergy,
    potential_energy: dyn PotentialEnergy
}



impl Mass {
    fn create_mass(mass: f64) -> Self {
        Mass { magnitude: mass }
    }
}

#[vectorify]
struct Force {}

impl ForceTrait for Force {}

impl  Force {
    fn new(acceleration: Acceleration, mass: Mass) -> Self {
        let mag = acceleration.magnitude * mass.magnitude;
        Force {magnitude: mag, angle: acceleration.angle}
    }
}


impl Momentum {
    fn standard(mass: Mass, velocity: Velocity) -> Self {
        Momentum { magnitude: mass.magnitude*velocity.magnitude , angle: velocity.angle }
    }
}

impl KineticEnergy {
    fn new(mass: Mass, velocity: Velocity) -> Self {
        KineticEnergy { magnitude: mass.magnitude*velocity.magnitude.powi(2)/2.0 }
        
    }
}

fn main() {
    let new_mass = Mass::create_mass(100.0);
    let new_accel = Acceleration { magnitude: 10.0, angle: 10.0};
    let first = Force::new(new_accel, new_mass);

    println!("{:#?}", first)

}
