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

    // what do i name this
    fn idk(x: f64) -> f64 {
        x * x * x * (x * (x * 6.0 - 15.0) + 10.0)
    }

    fn lerp(x: f64, y: f64, z: f64) -> f64 {
        y + x * (z - y)
    }

    fn grad(a: i32, x: f64, y: f64, z: f64) -> f64 {
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

        let temp1 = Self::idk(x);
        let temp2 = Self::idk(y);
        let temp3 = Self::idk(z);

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

    pub fn populate_noise_array(
        &self,
        var1: Vec<f64>,
        var2: f64,
        var4: f64,
        var6: f64,
        var8: f64,
        y: i32,
        var10: i32,
        z: f64,
        var13: f64,
        var15: f64,
        var17: f64,
    ) {
        let var19 = 0;
        let var20 = 1.0 / var17;
        let mut var22 = -1;
        let var23 = false;
        let var24 = false;
        let var25 = false;
        let var26 = false;
        let var27 = false;
        let var28 = false;
        let var29 = 0.0;
        let var31 = 0.0;
        let var33 = 0.0;
        let var35 = 0.0;

        for var37 in 0..var8.trunc() as i64 {
            let mut var38 = (var2 + var37 as f64) * z + self.x;
            let mut var40 = var38.trunc() as i64;
            if var38 < var40 as f64 {
                var40 -= 1;
            }

            let var41 = var40 & 255;
            var38 -= var40 as f64;
            let var42 = Self::idk(var38);

            for var44 in 0..var10 {
                let mut var45 = (var6 + var44 as f64) * var15 + self.z;
                let mut var47 = var45.trunc() as i64;
                if var45 < var47 as f64 {
                    var47 -= 1;
                }

                let var48 = var47 & 255;
                var45 -= var47 as f64;
                let var49 = Self::idk(var45);

                for var51 in 0..y {
                    let mut var52 = (var4 + var51 as f64) * var13 + self.y;
                    let mut var54 = var52.trunc() as i64;
                    if var52 < var54 as f64 {
                        var54 -= 1;
                    }

                    let var55 = var54 & 255;
                    var52 -= var54 as f64;
                    let var56 = Self::idk(var52);

                    if var51 == 0 || var55 != var22 {
                        var22 = var55;
                        let var64 = self.permutations[var41 as usize] + var55 as i32;
                        let var65 = self.permutations[var64 as usize] + var48 as i32;
                        let var66 = self.permutations[var64 as usize + 1] + var48 as i32;
                        let var67 = self.permutations[var41 as usize + 1] + var55 as i32;
                        let var68 = self.permutations[var67 as usize] + var48 as i32;
                        let var69 = self.permutations[var67 as usize + 1] + var48 as i32;
                    }
                }
            }
        }
    }
}
