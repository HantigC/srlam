use core::f64;

use nalgebra::{self as na, Vector3};
fn main() {
    let rotation_matrix = na::Matrix3::<f64>::identity();
    // let rotation_axis = na::Vector3::z_axis();
    let rotation_axis = na::Unit::new_normalize(na::Vector3::<f64>::new(
        0.0, 0.0, 1.0,
    ));
    let rotation_vector = na::Rotation3::from_axis_angle(
        &rotation_axis,
        f64::consts::FRAC_PI_4,
    );
    let rotation_matrix_vector = rotation_vector.matrix();
    println!(
        "rotation matrix from rotation vector = {}",
        rotation_matrix_vector
    );
    println!(
        "rotation matrix from rotation matrix = {}",
        rotation_matrix
    );
    let v1: na::Vector3<f64> = na::Vector3::new(1.0, 0.0, 0.0);
    let v1_rotated = rotation_vector * v1;
    println!("v1={}", v1);
    println!("v1_rotated={}", v1_rotated);
    let v1_rotated_matrix = rotation_matrix_vector * v1;
    println!(
        "v1_rotated_matrix={}",
        v1_rotated_matrix
    );

    let axi_angle = rotation_vector.scaled_axis();
    
    
    let mut transform = na::Isometry3::<f64>::identity();
    transform.append_rotation_mut(&na::UnitQuaternion::new(axi_angle));
    transform.append_translation_mut(&na::Translation3::new(1.0, 0.0, 0.0));
    println!("transform matrix={}", transform);
    let v1_iso = transform * v1;
    let p1 = na::Point3::new(v1.x, v1.y, v1.z);
    let p1_iso = transform.transform_point(&p1);
    println!("v1_iso={}", v1_iso);
    println!("p11_iso={}", p1_iso);
    println!("transform as matrix={}", transform.to_matrix());
    println!("v1_h{}", v1.to_homogeneous());
    println!("is_iso_mat={}", transform.to_matrix() * v1.to_homogeneous());
    let q = na::UnitQuaternion::new(axi_angle);
    println!("q={}", q);
    let rotated_p_with_q = q * p1;
    println!("rotated_p_with_q={}", rotated_p_with_q);
    let rotated_v_with_q = q * v1;
    println!("rotated_v_with_q={}", rotated_v_with_q);
    println!("underneath rotated_v_with_q={}", q.quaternion() * na::Quaternion::new(0.0, v1.x, v1.y, v1.z) * q.inverse().quaternion());
}
