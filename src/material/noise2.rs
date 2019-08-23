use crate::Vec3;
use lazy_static::lazy_static;
use rand::seq::SliceRandom;
use rand::thread_rng;

/// https://mrl.nyu.edu/~perlin/noise/

#[derive(Clone)]
pub struct Perlin;

impl Perlin {
    pub fn noise(p: Vec3) -> f32 {
        let x = p.x - p.x.floor();
        let y = p.y - p.y.floor();
        let z = p.z - p.z.floor();

        let xx = p.x.floor() as usize & 255;
        let yy = p.y.floor() as usize & 255;
        let zz = p.z.floor() as usize & 255;

        let (u, v, w) = (fade(x), fade(y), fade(z));

        let (a, b) = (P[xx] as usize + yy, P[xx + 1] as usize + yy);
        let (aa, ab) = (P[a] as usize + zz, P[a + 1] as usize + zz);
        let (ba, bb) = (P[b] as usize + zz, P[b + 1] as usize + zz);

        let u0 = lerp(
            grad(P[aa], Vec3::new(x, y, z)),
            grad(P[ba], Vec3::new(x - 1.0, y, z)),
            u,
        );
        let u1 = lerp(
            grad(P[ab], Vec3::new(x, y - 1.0, z)),
            grad(P[bb], Vec3::new(x - 1.0, y - 1.0, z)),
            u,
        );
        let u2 = lerp(
            grad(P[aa + 1], Vec3::new(x, y, z - 1.0)),
            grad(P[ba + 1], Vec3::new(x - 1.0, y, z - 1.0)),
            u,
        );
        let u3 = lerp(
            grad(P[ab + 1], Vec3::new(x, y - 1.0, z - 1.0)),
            grad(P[bb + 1], Vec3::new(x - 1.0, y - 1.0, z - 1.0)),
            u,
        );
        let v0 = lerp(u0, u1, v);
        let v1 = lerp(u2, u3, v);

        lerp(v0, v1, w)
    }

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
    // PERMUTATION
    static ref P: [u8; 512] = {
        let mut arr = [0u8; 512];
        (0..256).for_each(|i| arr[i] = i as u8);
        (&mut arr[0..256]).shuffle(&mut thread_rng());
        arr.copy_within(0..256, 256);
        arr
    };
}

fn fade(t: f32) -> f32 {
    t * t * t * (t * (t * 6.0 - 15.0) + 10.0)
}

fn grad(hash: u8, p: Vec3) -> f32 {
    let h = hash & 15;
    let u = if h < 8 { p.x } else { p.y };
    let u = if h & 1 == 0 { u } else { -u };

    let v = if h < 4 {
        p.y
    } else if h == 12 || h == 14 {
        p.x
    } else {
        p.z
    };
    let v = if h & 2 == 0 { v } else { -v };

    u + v
}

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + t * (b - a)
}
