//! Шум биомов: `LegacyRandomSource`, `SimplexNoise`, `PerlinSimplexNoise` (порт 1.21.11).

const MASK: i64 = 281_474_976_710_655;
const MULT: i64 = 25_214_903_917;

/// VANILLA: LegacyRandomSource (+ WorldgenRandom как прозрачная обёртка).
pub struct LegacyRandom {
    seed: i64,
    /// VANILLA: MarsagliaPolarGaussian (кэш второго значения).
    next_gaussian: Option<f64>,
}

impl LegacyRandom {
    pub fn new(seed: i64) -> Self {
        Self {
            seed: (seed ^ MULT) & MASK,
            next_gaussian: None,
        }
    }

    /// VANILLA: RandomSource#setSeed (сбрасывает гауссов кэш).
    pub fn set_seed(&mut self, seed: i64) {
        self.seed = (seed ^ MULT) & MASK;
        self.next_gaussian = None;
    }

    // VANILLA: BitRandomSource#nextFloat
    pub fn next_float(&mut self) -> f32 {
        self.next(24) as f32 * 5.960_464_5E-8
    }

    // VANILLA: MarsagliaPolarGaussian#nextGaussian
    pub fn next_gaussian(&mut self) -> f64 {
        if let Some(g) = self.next_gaussian.take() {
            return g;
        }
        loop {
            let x = 2.0 * self.next_double() - 1.0;
            let y = 2.0 * self.next_double() - 1.0;
            let r = x * x + y * y;
            if r < 1.0 && r != 0.0 {
                let m = (-2.0 * r.ln() / r).sqrt();
                self.next_gaussian = Some(y * m);
                return x * m;
            }
        }
    }

    pub fn next(&mut self, bits: u32) -> i32 {
        self.seed = (self.seed.wrapping_mul(MULT).wrapping_add(11)) & MASK;
        (self.seed >> (48 - bits)) as i32
    }

    // VANILLA: BitRandomSource#nextBoolean
    pub fn next_boolean(&mut self) -> bool {
        self.next(1) != 0
    }

    // VANILLA: BitRandomSource#nextInt(int)
    pub fn next_int_bound(&mut self, bound: i32) -> i32 {
        if bound & (bound - 1) == 0 {
            return ((i64::from(bound) * i64::from(self.next(31))) >> 31) as i32;
        }
        loop {
            let a = self.next(31);
            let b = a % bound;
            if a.wrapping_sub(b).wrapping_add(bound - 1) >= 0 {
                return b;
            }
        }
    }

    pub fn next_long(&mut self) -> i64 {
        let a = i64::from(self.next(32));
        let b = i64::from(self.next(32));
        (a << 32).wrapping_add(b)
    }

    pub fn next_double(&mut self) -> f64 {
        let a = i64::from(self.next(26));
        let b = i64::from(self.next(27));
        (((a << 27) + b) as f64) * f64::from(1.110_223E-16_f32)
    }

    // VANILLA: RandomSource#consumeCount
    pub fn consume_count(&mut self, n: u32) {
        for _ in 0..n {
            self.next(32);
        }
    }
}

const GRADIENT: [[i32; 3]; 16] = [
    [1, 1, 0],
    [-1, 1, 0],
    [1, -1, 0],
    [-1, -1, 0],
    [1, 0, 1],
    [-1, 0, 1],
    [1, 0, -1],
    [-1, 0, -1],
    [0, 1, 1],
    [0, -1, 1],
    [0, 1, -1],
    [0, -1, -1],
    [1, 1, 0],
    [0, -1, 1],
    [-1, 1, 0],
    [0, -1, -1],
];

fn floor(v: f64) -> i32 {
    v.floor() as i32
}

/// VANILLA: SimplexNoise
pub struct SimplexNoise {
    p: [i32; 512],
    pub xo: f64,
    pub yo: f64,
    pub zo: f64,
}

impl SimplexNoise {
    pub fn new(r: &mut LegacyRandom) -> Self {
        let xo = r.next_double() * 256.0;
        let yo = r.next_double() * 256.0;
        let zo = r.next_double() * 256.0;
        let mut p = [0i32; 512];
        for (i, v) in p.iter_mut().enumerate().take(256) {
            *v = i as i32;
        }
        for i in 0..256usize {
            let j = r.next_int_bound(256 - i as i32) as usize;
            p.swap(i, j + i);
        }
        Self { p, xo, yo, zo }
    }

    fn p(&self, i: i32) -> i32 {
        self.p[(i & 0xFF) as usize]
    }

    fn corner(g: usize, x: f64, y: f64, z: f64, t: f64) -> f64 {
        let mut a = t - x * x - y * y - z * z;
        if a < 0.0 {
            0.0
        } else {
            a *= a;
            let gr = GRADIENT[g];
            a * a * (f64::from(gr[0]) * x + f64::from(gr[1]) * y + f64::from(gr[2]) * z)
        }
    }

    pub fn get_value_2d(&self, x: f64, y: f64) -> f64 {
        let sqrt3 = 3.0f64.sqrt();
        let f2 = 0.5 * (sqrt3 - 1.0);
        let g2 = (3.0 - sqrt3) / 6.0;
        let s = (x + y) * f2;
        let i = floor(x + s);
        let j = floor(y + s);
        let t = f64::from(i + j) * g2;
        let x0 = x - (f64::from(i) - t);
        let y0 = y - (f64::from(j) - t);
        let (i1, j1) = if x0 > y0 { (1, 0) } else { (0, 1) };
        let x1 = x0 - f64::from(i1) + g2;
        let y1 = y0 - f64::from(j1) + g2;
        let x2 = x0 - 1.0 + 2.0 * g2;
        let y2 = y0 - 1.0 + 2.0 * g2;
        let ii = i & 0xFF;
        let jj = j & 0xFF;
        let g0 = (self.p(ii + self.p(jj)) % 12) as usize;
        let g1 = (self.p(ii + i1 + self.p(jj + j1)) % 12) as usize;
        let g2i = (self.p(ii + 1 + self.p(jj + 1)) % 12) as usize;
        70.0 * (Self::corner(g0, x0, y0, 0.0, 0.5)
            + Self::corner(g1, x1, y1, 0.0, 0.5)
            + Self::corner(g2i, x2, y2, 0.0, 0.5))
    }

    pub fn get_value_3d(&self, x: f64, y: f64, z: f64) -> f64 {
        let s = (x + y + z) * (1.0 / 3.0);
        let i = floor(x + s);
        let j = floor(y + s);
        let k = floor(z + s);
        let t = f64::from(i + j + k) * (1.0 / 6.0);
        let x0 = x - (f64::from(i) - t);
        let y0 = y - (f64::from(j) - t);
        let z0 = z - (f64::from(k) - t);
        let (i1, j1, k1, i2, j2, k2) = if x0 >= y0 {
            if y0 >= z0 {
                (1, 0, 0, 1, 1, 0)
            } else if x0 >= z0 {
                (1, 0, 0, 1, 0, 1)
            } else {
                (0, 0, 1, 1, 0, 1)
            }
        } else if y0 < z0 {
            (0, 0, 1, 0, 1, 1)
        } else if x0 < z0 {
            (0, 1, 0, 0, 1, 1)
        } else {
            (0, 1, 0, 1, 1, 0)
        };
        let (x1, y1, z1) = (
            x0 - f64::from(i1) + 1.0 / 6.0,
            y0 - f64::from(j1) + 1.0 / 6.0,
            z0 - f64::from(k1) + 1.0 / 6.0,
        );
        let (x2, y2, z2) = (
            x0 - f64::from(i2) + 1.0 / 3.0,
            y0 - f64::from(j2) + 1.0 / 3.0,
            z0 - f64::from(k2) + 1.0 / 3.0,
        );
        let (x3, y3, z3) = (x0 - 1.0 + 0.5, y0 - 1.0 + 0.5, z0 - 1.0 + 0.5);
        let (a, b, c) = (i & 0xFF, j & 0xFF, k & 0xFF);
        let g0 = (self.p(a + self.p(b + self.p(c))) % 12) as usize;
        let g1 = (self.p(a + i1 + self.p(b + j1 + self.p(c + k1))) % 12) as usize;
        let g2 = (self.p(a + i2 + self.p(b + j2 + self.p(c + k2))) % 12) as usize;
        let g3 = (self.p(a + 1 + self.p(b + 1 + self.p(c + 1))) % 12) as usize;
        32.0 * (Self::corner(g0, x0, y0, z0, 0.6)
            + Self::corner(g1, x1, y1, z1, 0.6)
            + Self::corner(g2, x2, y2, z2, 0.6)
            + Self::corner(g3, x3, y3, z3, 0.6))
    }
}

/// VANILLA: PerlinSimplexNoise
pub struct PerlinSimplexNoise {
    levels: Vec<Option<SimplexNoise>>,
    highest_freq_value_factor: f64,
    highest_freq_input_factor: f64,
}

impl PerlinSimplexNoise {
    /// `octaves` — как `ImmutableList.of(..)` в ванили (не пустой).
    pub fn new(r: &mut LegacyRandom, octaves: &[i32]) -> Self {
        let mut set: Vec<i32> = octaves.to_vec();
        set.sort_unstable();
        set.dedup();
        let first = set.first().copied().unwrap_or(0);
        let last = set.last().copied().unwrap_or(0);
        let a = -first;
        let b = last;
        let n = a + b + 1;
        let n_us = n.max(1) as usize;
        let mut levels: Vec<Option<SimplexNoise>> = (0..n_us).map(|_| None).collect();
        let base = SimplexNoise::new(r);
        let k = b;
        let (bxo, byo, bzo) = (base.xo, base.yo, base.zo);
        let base_ref = if k >= 0 && k < n && set.contains(&0) {
            Some(base)
        } else {
            None
        };
        let mut base_for_seed = None;
        if let Some(bn) = base_ref {
            base_for_seed = Some(bn.get_value_3d(bxo, byo, bzo));
            levels[k as usize] = Some(bn);
        }
        for i in (k + 1)..n {
            if i >= 0 && set.contains(&(k - i)) {
                levels[i as usize] = Some(SimplexNoise::new(r));
            } else {
                r.consume_count(262);
            }
        }
        if b > 0 {
            let v = base_for_seed.unwrap_or(0.0); // base не сохранён: ванильный код всё равно читает его значение
            let seed = (v * f64::from(9.223_372E18_f32)) as i64;
            let mut r2 = LegacyRandom::new(seed);
            for i in (0..k).rev() {
                if i < n && set.contains(&(k - i)) {
                    levels[i as usize] = Some(SimplexNoise::new(&mut r2));
                } else {
                    r2.consume_count(262);
                }
            }
        }
        Self {
            levels,
            highest_freq_input_factor: 2.0f64.powi(b),
            highest_freq_value_factor: 1.0 / (2.0f64.powi(n) - 1.0),
        }
    }

    pub fn get_value(&self, x: f64, y: f64, use_offsets: bool) -> f64 {
        let mut total = 0.0;
        let mut inp = self.highest_freq_input_factor;
        let mut val = self.highest_freq_value_factor;
        for s in self.levels.iter().flatten() {
            let (ox, oy) = if use_offsets {
                (s.xo, s.yo)
            } else {
                (0.0, 0.0)
            };
            total += s.get_value_2d(x * inp + ox, y * inp + oy) * val;
            inp /= 2.0;
            val *= 2.0;
        }
        total
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn java_random_matches_known_sequence() {
        // new java.util.Random(0).nextInt() == -1155484576; Legacy использует тот же LCG
        let mut r = LegacyRandom::new(0);
        assert_eq!(r.next(32), -1_155_484_576);
        assert_eq!(r.next(32), -723_955_400);
        // Random(42): nextInt() == -1170105035 → next(31) = 1562431130 → % 100 = 30
        let mut r = LegacyRandom::new(42);
        assert_eq!(r.next_int_bound(100), 30);
    }

    #[test]
    fn perlin_simplex_is_deterministic_and_bounded() {
        let mut a = LegacyRandom::new(2345);
        let n1 = PerlinSimplexNoise::new(&mut a, &[0]);
        let mut b = LegacyRandom::new(2345);
        let n2 = PerlinSimplexNoise::new(&mut b, &[0]);
        let mut seen_neg = false;
        let mut seen_pos = false;
        for i in 0..200 {
            let v = n1.get_value(f64::from(i) * 0.0225 * 7.0, f64::from(i) * 0.31, false);
            assert_eq!(
                v,
                n2.get_value(f64::from(i) * 0.0225 * 7.0, f64::from(i) * 0.31, false)
            );
            assert!(v.abs() <= 1.2);
            seen_neg |= v < -0.1;
            seen_pos |= v > 0.1;
        }
        assert!(seen_neg && seen_pos);
        let mut c = LegacyRandom::new(3456);
        let f = PerlinSimplexNoise::new(&mut c, &[-2, -1, 0]);
        assert!(f.get_value(1.5, 2.5, false).is_finite());
    }
}
