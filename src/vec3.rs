use std::ops;
use std::fmt;

pub struct vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl vec3 {
    fn length(&self) -> f64 {
        return (self.x * self.x + self.y * self.y + self.z * self.z).powf(0.5);
    }
}

impl fmt::Display for vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "{} {} {}", self.x, self.y, self.z);
    }
}

impl ops::Add<vec3> for vec3 {

    type Output = vec3;

    fn add(self, r: vec3) -> vec3 {
        return vec3{x: r.x + self.x, y: r.y + self.y, z: r.z + self.z}
    }
}

impl ops::Div<vec3> for vec3 {

    type Output = vec3;

    fn div(self, r: vec3) -> vec3 {
        return vec3{x: self.x / r.x, y: self.y / r.y, z: self.z / r.z}
    }
}

impl vec3 {
    fn dot(self, r: vec3) -> f64 {
        return self.x * r.x + self.y * r.y + self.z * r.z
    }
}

impl ops::Sub<vec3> for vec3 {

    type Output = vec3;

    fn sub(self, r: vec3) -> vec3 {
        return vec3{x: self.x - r.x, y: self.y - r.y, z: self.z - r.z}
    }
}

impl ops::Mul<vec3> for vec3 {
    type Output = vec3;

    fn mul(self, r: vec3) -> vec3 {
        return vec3{x: self.x * r.x, y: self.y * r.y, z: self.z * r.z}
    }
}

impl ops::Mul<f64> for vec3 {
    type Output = vec3;

    fn mul(self, f: f64) -> vec3 {
        return vec3{x: self.x * f, y: self.y * f, z: self.z * f}
    }
}
