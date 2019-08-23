#![allow(clippy::many_single_char_names)]
#![allow(clippy::needless_range_loop)]

use crate::Vec3;
use lazy_static::lazy_static;
use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};

/// perlin noise
#[derive(Clone)]
pub struct Perlin;

impl Perlin {
    /// return noise value at point
    pub fn noise(p: Vec3) -> f32 {
        let u = p.x - p.x.floor();
        let v = p.y - p.y.floor();
        let w = p.z - p.z.floor();

        let i = p.x.floor() as usize;
        let j = p.y.floor() as usize;
        let k = p.z.floor() as usize;

        let mut c = [[[Vec3::zero(); 2]; 2]; 2];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    let ii = PERM_X[(i + di) & 255];
                    let jj = PERM_Y[(j + dj) & 255];
                    let kk = PERM_Z[(k + dk) & 255];
                    c[di][dj][dk] = RAN_VEC[(ii ^ jj ^ kk) as usize];
                }
            }
        }

        perlin_interp(&c, u, v, w)
    }

    /// turbulence
    pub fn turb(p: Vec3) -> f32 {
        let mut acc = 0.0;
        let mut p = p;
        let mut weight = 1.0;

        for _ in 0..7 {
            acc += weight * Perlin::noise(p);
            weight *= 0.5;
            p *= 2.0;
        }

        acc.abs()
    }
}

lazy_static! {
    static ref PERM_X: [u8; 256] = generate_perm();
    static ref PERM_Y: [u8; 256] = generate_perm();
    static ref PERM_Z: [u8; 256] = generate_perm();
    static ref RAN_VEC: [Vec3; 256] = generate_perlin();
}

fn generate_perm() -> [u8; 256] {
    let mut arr = [0u8; 256];
    (0..256).for_each(|i| arr[i] = i as u8);
    arr.shuffle(&mut thread_rng());
    arr
}

fn generate_perlin() -> [Vec3; 256] {
    let mut arr = [Vec3::zero(); 256];
    (0..256).for_each(|i| {
        arr[i] = Vec3::new(
            thread_rng().gen_range(-1.0, 1.0),
            thread_rng().gen_range(-1.0, 1.0),
            thread_rng().gen_range(-1.0, 1.0),
        )
        .normalize();
    });
    arr
}

fn perlin_interp(c: &[[[Vec3; 2]; 2]; 2], u: f32, v: f32, w: f32) -> f32 {
    let uu = u * u * (3.0 - 2.0 * u);
    let vv = v * v * (3.0 - 2.0 * v);
    let ww = w * w * (3.0 - 2.0 * w);

    let mut acc = 0.0;
    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                let weighted = Vec3::new(u - i as f32, v - j as f32, w - k as f32);
                acc += (i as f32 * uu + (1.0 - i as f32) * (1.0 - uu))
                    * (j as f32 * vv + (1.0 - j as f32) * (1.0 - vv))
                    * (k as f32 * ww + (1.0 - k as f32) * (1.0 - ww))
                    * c[i][j][k].dot(weighted);
            }
        }
    }

    acc
}
