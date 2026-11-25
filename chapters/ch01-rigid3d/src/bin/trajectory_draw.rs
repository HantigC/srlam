use core::fmt;
use kiss3d::prelude as k3d;
use nalgebra as na;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct TimestampedRigid3D {
    timestamp: f64,
    pose: na::Isometry3<f32>,
}

impl TimestampedRigid3D {
    pub fn new(
        timestamp: f64,
        pose: na::Isometry3<f32>,
    ) -> Self {
        Self { timestamp, pose }
    }
}
impl fmt::Display for TimestampedRigid3D {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(
            f,
            "time={}, pose={}",
            self.timestamp, self.pose
        )
    }
}

fn read_trajectory(filepath: &str) -> Vec<TimestampedRigid3D> {
    let file = File::open(filepath).expect("The filepath doesn't exist {msg}");
    let reader = BufReader::new(file);
    let mut trajectory: Vec<TimestampedRigid3D> = Vec::new();
    for line in reader.lines() {
        let line = line.expect("bad line");
        let nums: Vec<f64> = line
            .split_whitespace()
            .map(|s| s.parse::<f64>().unwrap())
            .collect();
        let q = na::UnitQuaternion::from_quaternion(na::Quaternion::new(
            nums[4] as f32,
            nums[5] as f32,
            nums[6] as f32,
            nums[7] as f32,
        ));
        let t = na::Translation3::new(
            nums[1] as f32,
            nums[2] as f32,
            nums[3] as f32,
        );
        let pose = na::Isometry3::from_parts(t, q);
        let timed_pose = TimestampedRigid3D::new(nums[0], pose);
        trajectory.push(timed_pose);
    }
    trajectory
}

fn plot_pose<'a, 'b>(
    window: &'a mut k3d::Window,
    timed_pose: &'b TimestampedRigid3D,
) -> &'a k3d::Window {
    let xaxis = na::Point3::new(0.1, 0.0, 0.0);
    let yaxis = na::Point3::new(0.0, 0.1, 0.0);
    let zaxis = na::Point3::new(0.0, 0.0, 0.1);

    let w_t = timed_pose.pose.translation;
    let w_xaxis = timed_pose.pose * xaxis;
    let w_yaxis = timed_pose.pose * yaxis;
    let w_zaxis = timed_pose.pose * zaxis;
    window.draw_line(
        k3d::Vec3::new(w_t.x, w_t.y, w_t.z),
        k3d::Vec3::new(
            w_xaxis.x, w_xaxis.y, w_xaxis.z,
        ),
        k3d::RED,
        4.0,
        true,
    );
    window.draw_line(
        k3d::Vec3::new(w_t.x, w_t.y, w_t.z),
        k3d::Vec3::new(
            w_yaxis.x, w_yaxis.y, w_yaxis.z,
        ),
        k3d::GREEN,
        4.0,
        true,
    );
    window.draw_line(
        k3d::Vec3::new(w_t.x, w_t.y, w_t.z),
        k3d::Vec3::new(
            w_zaxis.x, w_zaxis.y, w_zaxis.z,
        ),
        k3d::BLUE,
        4.0,
        true,
    );
    window
}

fn plot_poses<'a, 'b>(
    window: &'a mut k3d::Window,
    timed_poses: &'b Vec<TimestampedRigid3D>,
) -> &'a k3d::Window {
    for timed_pose in timed_poses.iter() {
        plot_pose(window, timed_pose);
    }
    window
}

#[kiss3d::main]
async fn main() {
    let filepath = "resources/ch1/trajectory.txt";
    let trajectory = read_trajectory(filepath);
    let mut window = k3d::Window::new("Trajectory").await;
    let mut camera = k3d::OrbitCamera3d::default();
    let mut scene = k3d::SceneNode3d::empty();
    scene
        .add_light(k3d::Light::point(100.0))
        .set_position(k3d::Vec3::new(0.0, 0.0, 0.0));
    while window.render_3d(&mut scene, &mut camera).await {
        plot_poses(&mut window, &trajectory);
    }
}
