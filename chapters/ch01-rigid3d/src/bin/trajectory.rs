use nalgebra as na;
fn main() {
    let mut q1 = na::Quaternion::<f32>::new(0.35, 0.2, 0.3, 0.1);
    let mut q2 = na::Quaternion::<f32>::new(-0.5, 0.4, -0.1, 0.2);
    q1.normalize_mut();
    q2.normalize_mut();

    na::UnitQuaternion::from_quaternion(q2);

    q2.normalize_mut();
    let t1 = na::Vector3::new(0.3, 0.1, 0.1);
    let t2 = na::Vector3::new(-0.1, 0.5, 0.3);
    let p1 = na::Point3::new(0.5, 0.0, 0.2);
    let t1w = na::Isometry3::from_parts(
        na::Translation3::from(t1),
        na::UnitQuaternion::from_quaternion(q1),
    );
    let t2w = na::Isometry3::from_parts(
        na::Translation3::from(t2),
        na::UnitQuaternion::from_quaternion(q2),
    );
    let p2 = t2w * t1w.inverse() * p1;
    println!("p2={}", p2);
}
