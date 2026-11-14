use nalgebra as na;
fn main() {
    let v: na::Vector3<f64> = na::Vector3::new(1.0, 2.0, 3.0);
    let m13: na::Matrix1x3<f64> = na::Matrix1x3::zeros();
    let z33: na::Matrix3<f64> = na::Matrix3::<f64>::zeros();
    println!("vector = {}", v);
    println!("m13 = {}", m13);
    println!("z33 = {}", z33);
    let dyn33: na::DMatrix<f64> = na::DMatrix::<f64>::from_fn(3, 3, |x, y| {
        x as f64 + y as f64 * 3.0
    });
    println!("dyn33 = {}", dyn33);
    println!(
        "det(dyn33) = {}",
        dyn33.determinant()
    );
    let identity33 = na::Matrix3::<f64>::identity();
    println!("identity33 = {}", identity33);
    println!(
        "det(identity33) = {}",
        identity33.determinant()
    );
    let dyn33_t = dyn33.transpose();
    let dyn33_inv = dyn33.try_inverse().unwrap_or(na::DMatrix::<f64>::identity(
        3, 3,
    ));
    println!("dyn33^T = {}", dyn33_t);
    println!("dyn33^-1 = {}", dyn33_inv);
    let diag33 = na::Matrix3::<f64>::from_diagonal_element(5.0);
    println!("diag33 = {}", diag33);
    let diag33_qr = diag33.qr();
    println!("diag33_q = {}", diag33_qr.q());
    println!("diag33_r = {}", diag33_qr.r());
    let diag33_eigenvalues = diag33.eigenvalues().unwrap();
    let diag33_eigenvectors = diag33.symmetric_eigen();
    println!(
        "diag33_eigenvalues = {}",
        diag33_eigenvalues
    );
    println!(
        "diag33_eigenvectors = {}",
        diag33_eigenvectors.eigenvectors
    );
    println!(
        "diag33_eigenvalues = {}",
        diag33_eigenvectors.eigenvalues
    );
    let diag33_svd = diag33.svd(true, true);
    println!(
        "diag33_svd_u = {}",
        diag33_svd.u.unwrap(),
    );
    println!(
        "diag33_svd_v^T = {}",
        diag33_svd.v_t.unwrap(),
    );
    println!(
        "diag33_svd_s = {}",
        diag33_svd.singular_values,
    );
}
