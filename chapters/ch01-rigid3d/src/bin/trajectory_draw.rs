use core::{fmt, num};
use std::fmt::write;
use nalgebra as na;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Rigid3D {
    translation: na::Translation3<f64>,
    rotation: na::UnitQuaternion<f64>,
}
impl fmt::Display for Rigid3D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "translation={}, rot={}", self.translation, self.rotation)
    }
}

impl Rigid3D {
    pub fn new(
        translation: na::Translation3<f64>,
        rotation: na::UnitQuaternion<f64>,
    ) -> Self{
        Self { translation, rotation }
    }
}


#[derive(Debug)]
struct TimestampedRigid3D {
    timestamp: f64,
    pose: Rigid3D,
}

impl TimestampedRigid3D {
    pub fn new(

    timestamp: f64,
    pose: Rigid3D,
    ) -> Self {
        Self { timestamp, pose }
    }
}
impl fmt::Display for TimestampedRigid3D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "time={}, pose={}", self.timestamp, self.pose)
    }
}
fn main() {
    let filepath = "resources/ch1/trajectory.txt";
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
            nums[4], nums[5], nums[6], nums[7],
        ));
        let t = na::Translation3::new(nums[1], nums[2], nums[3]);
        let pose = Rigid3D::new(t, q);
        let timed_pose = TimestampedRigid3D::new(nums[0], pose);
        trajectory.push(timed_pose);
    }
    println!("trajectory {}", trajectory[0]);
}
