
use std::f32;
use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Div;
use std::ops::DivAssign;
use std::ops::Index;
use std::ops::IndexMut;
use std::ops::Mul;
use std::ops::MulAssign;
use std::ops::Sub;
use std::ops::SubAssign;

#[derive(Debug, Copy, Clone)]
pub struct Ray {
    pub origin: Vector3,
    pub direction: Vector3,
}

impl Ray {
    pub fn new(origin: Vector3, direction: Vector3) -> Ray {
        Ray {
            origin,
            direction
        }
    }

    pub fn get_origin(&self) -> Vector3 {
        self.origin
    }

    pub fn get_direction(&self) -> Vector3 {
        self.direction
    }

    pub fn get_point_at_time(&self, t: f32) -> Vector3 {
        self.direction * t
    }

}

#[derive(Debug, Copy, Clone)]
pub struct Vector3 {
    values: [f32; 3],
}

impl Vector3 {
    pub fn empty_new() -> Vector3 {
        Vector3 {
            values: [0.0, 0.0, 0.0],
        }
    }

    pub fn new(x: f32, y: f32, z: f32) -> Vector3 {
        Vector3 { values: [x, y, z] }
    }

    pub fn x(&self) -> &f32 {
        &self.values[0]
    }

    pub fn y(&self) -> &f32 {
        &self.values[1]
    }

    pub fn z(&self) -> &f32 {
        &self.values[2]
    }

    pub fn r(&self) -> &f32 {
        &self.values[0]
    }

    pub fn g(&self) -> &f32 {
        &self.values[1]
    }

    pub fn b(&self) -> &f32 {
        &self.values[2]
    }

    pub fn length(&self) -> f32 {
        ((&self.values[0]).powi(2) + (&self.values[1]).powi(2) + (&self.values[2]).powi(2))
            .sqrt()
    }

    pub fn squared_length(&self) -> f32 {
        &self.values[0] * &self.values[0] + &self.values[1] * &self.values[1] + &self.values[2] * &self.values[2]
    }

    pub fn make_unit_vector(&mut self) {
        let length = &self.length();
        self.values = [
            self.values[0] / length,
            self.values[1] / length,
            self.values[2] / length,
        ];
    }

    pub fn dot_with(&self, other: &Vector3) -> f32 {
        &self.values[0] * other.values[0]
            + &self.values[1] * other.values[1]
            + &self.values[2] * other.values[2]
    }

    pub fn cross_with(&self, other: &Vector3) -> Vector3 {
        Vector3 {
            values: [
                &self.values[1] * other.values[2] - &self.values[2] * other.values[1],
                &self.values[2] * other.values[0] - &self.values[0] * other.values[2],
                &self.values[0] * other.values[1] - &self.values[1] * other.values[0],
            ],
        }
    }
}

impl Add for Vector3 {
    type Output = Vector3;

    fn add(self, other: Vector3) -> Vector3 {
        Vector3 {
            values: [
                self.values[0] + other.values[0],
                self.values[1] + other.values[1],
                self.values[2] + other.values[2],
            ],
        }
    }
}

impl Add for &Vector3 {
    type Output = Vector3;

    fn add(self, other: &Vector3) -> Vector3 {
        Vector3 {
            values: [
                self.values[0] + other.values[0],
                self.values[1] + other.values[1],
                self.values[2] + other.values[2],
            ],
        }
    }
}

impl AddAssign for Vector3 {
    fn add_assign(&mut self, other: Vector3) {
        *self = Vector3 {
            values: [
                self.values[0] + other.values[0],
                self.values[1] + other.values[1],
                self.values[2] + other.values[2],
            ],
        };
    }
}

impl Sub for Vector3 {
    type Output = Vector3;

    fn sub(self, other: Vector3) -> Vector3 {
        Vector3 {
            values: [
                self.values[0] - other.values[0],
                self.values[1] - other.values[1],
                self.values[2] - other.values[2],
            ],
        }
    }
}

impl SubAssign for Vector3 {
    fn sub_assign(&mut self, other: Vector3) {
        *self = Vector3 {
            values: [
                self.values[0] - other.values[0],
                self.values[1] - other.values[1],
                self.values[2] - other.values[2],
            ],
        };
    }
}
impl Mul<f32> for Vector3 {
    type Output = Vector3;

    fn mul(self, scalar: f32) -> Vector3 {
        Vector3 {
            values: [
                self.values[0] * scalar,
                self.values[1] * scalar,
                self.values[2] * scalar,
            ],
        }
    }
}

impl Mul<f32> for &Vector3 {
    type Output = Vector3;

    fn mul(self, scalar: f32) -> Vector3 {
        Vector3 {
            values: [
                self.values[0] * scalar,
                self.values[1] * scalar,
                self.values[2] * scalar,
            ],
        }
    }
}

impl Div<f32> for Vector3 {
    type Output = Vector3;

    fn div(self, scalar: f32) -> Vector3 {
        Vector3 {
            values: [
                self.values[0] / scalar,
                self.values[1] / scalar,
                self.values[2] / scalar,
            ],
        }
    }
}

impl Div<f32> for &Vector3 {
    type Output = Vector3;

    fn div(self, scalar: f32) -> Vector3 {
        Vector3 {
            values: [
                self.values[0] / scalar,
                self.values[1] / scalar,
                self.values[2] / scalar,
            ],
        }
    }
}

impl MulAssign for Vector3 {
    fn mul_assign(&mut self, other: Vector3) {
        *self = Vector3 {
            values: [
                self.values[0] * other.values[0],
                self.values[1] * other.values[1],
                self.values[2] * other.values[2],
            ],
        };
    }
}

impl MulAssign<f32> for Vector3 {
    fn mul_assign(&mut self, other: f32) {
        *self = Vector3 {
            values: [
                self.values[0] * other,
                self.values[1] * other,
                self.values[2] * other,
            ],
        };
    }
}

impl DivAssign for Vector3 {
    fn div_assign(&mut self, other: Vector3) {
        *self = Vector3 {
            values: [
                self.values[0] / other.values[0],
                self.values[1] / other.values[1],
                self.values[2] / other.values[2],
            ],
        }
    }
}

impl DivAssign<f32> for Vector3 {
    fn div_assign(&mut self, other: f32) {
        *self = Vector3 {
            values: [
                self.values[0] / other,
                self.values[1] / other,
                self.values[2] / other,
            ],
        };
    }
}

impl Index<usize> for Vector3 {
    type Output = f32;

    fn index(&self, i: usize) -> &f32 {
        &self.values[i]
    }
}

impl IndexMut<usize> for Vector3 {
    fn index_mut(&mut self, i: usize) -> &mut f32 {
        &mut self.values[i]
    }
}

pub fn unit_vector(v: Vector3) -> Vector3 {
    v / v.length()
}

pub fn dot(v1: &Vector3, v2: &Vector3) -> f32 {
    v1.x() * v2.x() + v1.y() * v2.y() + v1.z() * v2.z()
}

/*
pub fn cross(&self, other: &Vector3) -> Vector3 {
    Vector3 {
        values: vec![
            &self.values[1] * other.values[2] - &self.values[2] * other.values[1],
            &self.values[2] * other.values[0] - &self.values[0] * other.values[2],
            &self.values[0] * other.values[1] - &self.values[1] * other.values[0],
        ],
    }
} */

