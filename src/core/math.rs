use glam::Vec3;
use rand::RngCore;
use std::f32::consts::PI;

pub trait Vec3Ext {
    fn near_zero(self) -> bool;
}
impl Vec3Ext for Vec3 {
    #[inline]
    fn near_zero(self) -> bool {
        self.length_squared() < 1e-12 // (1e-6)^2
    }
}

// Fast uniform [0,1)
#[inline]
pub fn f32_01(rng: &mut dyn RngCore) -> f32 {
    // 24 random bits -> float in [0,1)
    let v = (rng.next_u32() >> 8) as u32; // keep top 24 bits
    v as f32 * (1.0 / (1u32 << 24) as f32)
}

pub fn random_unit_vector(rng: &mut dyn RngCore) -> Vec3 {
    let a = 2.0 * PI * f32_01(rng); // [0, 2π)
    let z = 2.0 * f32_01(rng) - 1.0; // [-1, 1)
    let r = (1.0 - z * z).sqrt();
    Vec3::new(r * a.cos(), r * a.sin(), z)
}

pub fn random_in_unit_disk(rng: &mut dyn RngCore) -> Vec3 {
    loop {
        let x = 2.0 * f32_01(rng) - 1.0;
        let y = 2.0 * f32_01(rng) - 1.0;
        let p = Vec3::new(x, y, 0.0);
        if p.length_squared() < 1.0 {
            return p;
        }
    }
}

#[inline]
fn linear_to_gamma(v: f32) -> f32 {
    if v > 0.0 {
        v.sqrt()
    } else {
        0.0
    }
}

/// Convert linear‑RGB Vec3 (0‥1) -> 0x00RRGGBB for minifb
pub fn vec_to_rgb_u32(v: Vec3) -> u32 {
    // linear -> gamma-2
    let (r, g, b) = (
        linear_to_gamma(v.x),
        linear_to_gamma(v.y),
        linear_to_gamma(v.z),
    );

    let ir = (255.999 * r.clamp(0.0, 0.999)) as u32;
    let ig = (255.999 * g.clamp(0.0, 0.999)) as u32;
    let ib = (255.999 * b.clamp(0.0, 0.999)) as u32;

    // pack as 0x00RRGGBB  (minifb ignores the top byte on all platforms)
    (ir << 16) | (ig << 8) | ib
}
