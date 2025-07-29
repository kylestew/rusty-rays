use rand::Rng;

#[derive(Default, Debug, Clone, Copy)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    // '&self' is shorthand for 'self: &self'
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        self.x.abs() < s && self.y.abs() < s && self.z.abs() < s
    }

    pub fn dot(u: Vec3, v: Vec3) -> f64 {
        u.x * v.x + u.y * v.y + u.z * v.z
    }

    pub fn cross(u: Vec3, v: Vec3) -> Vec3 {
        Vec3 {
            x: u.y * v.z - u.z * v.y,
            y: u.z * v.x - u.x * v.z,
            z: u.x * v.y - u.y * v.x,
        }
    }

    pub fn unit_vector(v: Vec3) -> Vec3 {
        v / v.length()
    }

    pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
        v - 2.0 * Self::dot(v, n) * n
    }
}

pub type Point3 = Vec3;
pub type Color = Vec3;

#[inline]
fn linear_to_gamma(v: f64) -> f64 {
    if v > 0.0 {
        v.sqrt()
    } else {
        0.0
    }
}

impl Vec3 {
    /// Convert linear‑RGB Vec3 (0‥1) -> 0x00RRGGBB for minifb
    pub fn to_rgb_u32(&self) -> u32 {
        // linear -> gamma-2
        let (r, g, b) = (
            linear_to_gamma(self.x),
            linear_to_gamma(self.y),
            linear_to_gamma(self.z),
        );

        let ir = (255.999 * r.clamp(0.0, 0.999)) as u32;
        let ig = (255.999 * g.clamp(0.0, 0.999)) as u32;
        let ib = (255.999 * b.clamp(0.0, 0.999)) as u32;

        // pack as 0x00RRGGBB  (minifb ignores the top byte on all platforms)
        (ir << 16) | (ig << 8) | ib
    }
}

impl std::ops::Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Vec3 {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl std::ops::Add for Vec3 {
    type Output = Vec3;
    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl std::ops::Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl std::ops::Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl std::ops::Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, t: f64) -> Vec3 {
        Vec3 {
            x: self.x * t,
            y: self.y * t,
            z: self.z * t,
        }
    }
}

impl std::ops::Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, v: Vec3) -> Vec3 {
        v * self
    }
}

impl std::ops::Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, t: f64) -> Vec3 {
        self * (1.0 / t)
    }
}

impl std::ops::AddAssign for Vec3 {
    fn add_assign(&mut self, t: Self) {
        self.x += t.x;
        self.y += t.y;
        self.z += t.z;
    }
}

// RANDOM helpers
impl Vec3 {
    pub fn random_in_range(min: f64, max: f64) -> Self {
        let mut rng = rand::thread_rng();
        Vec3 {
            x: rng.gen_range(min..=max),
            y: rng.gen_range(min..=max),
            z: rng.gen_range(min..=max),
        }
    }

    pub fn random_unit_vector() -> Self {
        loop {
            let p = Vec3::random_in_range(-1.0, 1.0);
            let lensq = p.length_squared();
            if lensq > 1e-160 && lensq <= 1.0 {
                return p / lensq.sqrt();
            }
        }
    }

    pub fn random_on_hemisphere(normal: Vec3) -> Vec3 {
        let on_unit_sphere = Vec3::random_unit_vector();
        if Vec3::dot(on_unit_sphere, normal) > 0.0 {
            on_unit_sphere
        } else {
            -on_unit_sphere
        }
    }
}
