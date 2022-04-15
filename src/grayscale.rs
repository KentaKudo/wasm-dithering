use js_sys::Uint8ClampedArray;
use rand::{rngs::SmallRng, Rng, SeedableRng};

pub struct Grayscale {
    pixels: Vec<(f64, u8)>,
    width: usize
}

impl From<(Vec<u8>, usize)> for Grayscale {
    fn from((v, width): (Vec<u8>, usize)) -> Self {
        Self {
            pixels: v
                .chunks_exact(4)
                .map(|pxl| (
                    luminance(
                        srgb_to_linear(pxl[0] as f64 / 255.0),
                        srgb_to_linear(pxl[1] as f64 / 255.0),
                        srgb_to_linear(pxl[2] as f64 / 255.0),
                    ),
                    pxl[3]
                ))
                .collect(),
            width
        }
    }
}

impl Grayscale {
    pub fn quantise(&self) -> Self {
        let pixels = self.pixels.clone().iter()
            .map(|(l, alpha)| (if *l > 0.5 { 1.0 } else { 0.0 }, *alpha))
            .collect();

        Self { pixels, width: self.width }
    }
}

impl Grayscale {
    pub fn white_noise(&self) -> Result<Self, Box<dyn std::error::Error>> {
        let mut rng = SmallRng::from_rng(rand::thread_rng())?;
        let pixels = self.pixels.clone().iter()
            .map(|(l, alpha)| {
                let noise = rng.gen_range(0.0..1.0);
                (if *l > noise { 1.0 } else { 0.0 }, *alpha)
            })
            .collect();

        Ok(Self { pixels, width: self.width })
    }
}

impl Grayscale {
    pub fn bayer(&self, n: u64) -> Self {
        let matrix = bayer_matrix(n);
        let m_size = 2_usize.pow(n as u32 + 1);

        let pixels = self.pixels.clone().iter().enumerate()
            .map(|(i, (l, alpha))| {
                let (x, y) = (i % self.width, i / self.width);
                let idx = (x % m_size) * m_size + y % m_size;
                let b = matrix[idx] as f64 / 2.0_f64.powi(2 * n as i32 + 2);
                (if *l > b { 1.0 } else { 0.0 }, *alpha)
            })
            .collect();

        Self { pixels, width: self.width }
    }
}

impl Grayscale {
    pub fn floyd_steinberg(&self) -> Self {
        let (width, height) = (self.width, self.pixels.len() / self.width);

        let mut pixels = self.pixels.clone();
        for i in 0..pixels.len() {
            let (old, new) = (pixels[i].0, if pixels[i].0 > 0.5 { 1.0 } else { 0.0 });
            pixels[i].0 = new;

            let qerr = old - new;

            let (x, y) = (i % width, i / width);

            if x + 1 < width {
                pixels[i + 1].0 += qerr * 7.0 / 16.0;
            }

            if x != 0 && y + 1 < height {
                pixels[i + width - 1].0 += qerr * 3.0 / 16.0;
            }

            if y + 1 < height {
                pixels[i + width].0 += qerr * 5.0 / 16.0;
            }

            if x + 1 < width && y + 1 < height {
                pixels[i + width + 1].0 += qerr * 1.0 / 16.0;
            }
        }

        Self { pixels, width }
    }
}

impl From<Grayscale> for Uint8ClampedArray {
    fn from(g: Grayscale) -> Self {
        let result: Vec<u8> = g.pixels.iter()
            .flat_map(|(l, alpha)| {
                let srgb = (linear_to_srgb(*l) * 255.0).round() as u8;
                [srgb, srgb, srgb, *alpha]
            })
            .collect();
        
        result[..].into()
    }
}

fn bayer_matrix(n: u64) -> Vec<u64> {
    if n == 0 {
        return vec![0, 2, 3, 1];
    }

    let prev = bayer_matrix(n - 1);

    let m_size = 2_u64.pow(n as u32 + 1);
    (0..m_size * m_size)
        .map(|i| {
            let (x, y) = (i % m_size, i / m_size);
            let c = match (x / (2 * n), y / (2 * n)) {
                (0, 0) => 0,
                (1, 0) => 2,
                (0, 1) => 3,
                (1, 1) => 1,
                _ => 0,
            };

            let idx = (y % (2 * n)) * (2 * n) + (x % (2 * n));
            4 * prev[idx as usize] + c
        })
        .collect()
}

const GAMMA: f64 = 2.4;

fn srgb_to_linear(c: f64) -> f64 {
    if c < 0.04045 { c / 12.92 } else { ((c + 0.055) / 1.055).powf(GAMMA) }
}

fn linear_to_srgb(c: f64) -> f64 {
    if c <= 0.0031308 { 12.92 * c } else { 1.055 * c.powf(1.0 / GAMMA) - 0.055 }
}

fn luminance(r: f64, g: f64, b: f64) -> f64 {
    0.2126 * r + 0.7152 * g + 0.0722 * b
}
