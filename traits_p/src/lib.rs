pub trait Vector {
    fn get_x_component(&self) -> f64;
    fn get_y_component(&self) -> f64;
    fn get_angle(&self) -> f64;
    fn get_magnitude(&self) -> f64;
}

pub trait Scalar {
    fn get_magnitude(&self) -> f64;
}

pub trait UnitDisplay {}