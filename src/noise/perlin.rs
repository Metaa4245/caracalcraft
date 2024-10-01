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

    pub fn lerp(x: f64, y: f64, z: f64) -> f64 {
        y + x * (z - y)
    }

    pub fn grad(a: i32, x: f64, y: f64, z: f64) -> f64 {
        let a = a & 15;
        let b = if a < 8 { x } else { y };
        let c = if a < 4 {
            y
        } else if a != 12 && a != 14 {
            z
        } else {
            x
        };
        let temp1 = if a & 1 == 0 { b } else { -b };
        let temp2 = if a & 2 == 0 { c } else { -c };
        temp1 + temp2
    }

    pub fn generate_noise(&self, x: f64, y: f64, z: f64) -> f64 {
        let mut x = x + self.x;
        let mut y = y + self.y;
        let mut z = z + self.z;

        let x_trunc = x.trunc() as u64;
        let y_trunc = y.trunc() as u64;
        let z_trunc = z.trunc() as u64;

        if x < x_trunc as f64 {
            x -= 1.0;
        }

        if y < y_trunc as f64 {
            y -= 1.0;
        }

        if z < z_trunc as f64 {
            z -= 1.0;
        }

        let x_and = x_trunc & 255;
        let y_and = y_trunc & 255;
        let z_and = z_trunc & 255;

        x -= x_trunc as f64;
        y -= y_trunc as f64;
        z -= z_trunc as f64;

        let temp1 = x * x * x * (x * (x * 6.0 - 15.0) + 10.0);
        let temp2 = y * y * y * (y * (y * 6.0 - 15.0) + 10.0);
        let temp3 = z * z * z * (z * (z * 6.0 - 15.0) + 10.0);

        let perm1 = self.permutations[x_and as usize] + y_and as i32;
        let perm2 = self.permutations[perm1 as usize] + z_and as i32;
        let perm3 = self.permutations[perm1 as usize + 1] + z_and as i32;
        let perm4 = self.permutations[x_and as usize + 1] + y_and as i32;
        let perm5 = self.permutations[perm4 as usize] + z_and as i32;
        let perm6 = self.permutations[perm4 as usize + 1] + z_and as i32;

        // wow
        Self::lerp(
            temp3,
            Self::lerp(
                temp2,
                Self::lerp(
                    temp1,
                    Self::grad(self.permutations[perm2 as usize], x, y, z),
                    Self::grad(self.permutations[perm5 as usize], x - 1.0, y, z),
                ),
                Self::lerp(
                    temp1,
                    Self::grad(self.permutations[perm3 as usize], x, y - 1.0, z),
                    Self::grad(self.permutations[perm6 as usize], x - 1.0, y - 1.0, z),
                ),
            ),
            Self::lerp(
                temp2,
                Self::lerp(
                    temp1,
                    Self::grad(self.permutations[perm2 as usize + 1], x, y, z - 1.0),
                    Self::grad(self.permutations[perm5 as usize + 1], x - 1.0, y, z - 1.0),
                ),
                Self::lerp(
                    temp1,
                    Self::grad(self.permutations[perm3 as usize + 1], x, y - 1.0, z - 1.0),
                    Self::grad(
                        self.permutations[perm6 as usize + 1],
                        x - 1.0,
                        y - 1.0,
                        z - 1.0,
                    ),
                ),
            ),
        )
    }
}
