use lcg_rand::rand::LCG;

pub struct Perlin {
    permutations: Vec<i32>,
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Perlin {
    pub fn new() -> Self {
        let mut random: LCG = LCG::new();
        let mut perms: Vec<i32> = vec![0; 512];
        let x = random.next() as f64 * 256.0;
        let y = random.next() as f64 * 256.0;
        let z = random.next() as f64 * 256.0;

        for (i, p) in perms.iter_mut().enumerate().take(256) {
            *p = i as i32;
        }

        for i in 0..256 {
            let x = random.range(0, 256 - i) + i;
            let x = x as i64;
            perms.swap(i as usize, x as usize);
            perms[i as usize + 256] = perms[i as usize];
        }

        Self {
            permutations: perms,
            x,
            y,
            z,
        }
    }
}
