use core::f64;

use nalgebra as na;
fn main(){
    let rotation_matrix = na::Matrix3::<f64>::identity();
    // let rotation_axis = na::Vector3::z_axis();
    let rotation_axis = na::Unit::new_normalize(na::Vector3::<f64>::new(0.0, 0.0, 1.0));
    let rotation_vector = na::Rotation3::from_axis_angle(&rotation_axis, f64::consts::FRAC_PI_4);
    println!("rotation matrix from rotation vector = {}", rotation_vector.matrix());
    println!("rotation matrix from rotation matrix = {}", rotation_matrix);
}