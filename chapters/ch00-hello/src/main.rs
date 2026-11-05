use nalgebra::{Matrix3, Vector3};

fn main() {
    let v: Vector3<f64> = Vector3::new(1.0, 2.0, 3.0);
    let m: Matrix3<f64> = Matrix3::new(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0);
    let result = m * v;
    
    println!("Hello, world!");
    print!("{}", result)
}
