pub mod canvas {
    use std::{
        fs::File,
        io::{self, BufWriter, Write},
    };

    pub struct Canvas {
        pixels: Vec<Vec<u32>>,
        width: usize,
        height: usize,
        _stride: usize,
    }

    impl Canvas {
        pub fn new(width: usize, height: usize) -> Canvas {
            let pixels = vec![vec![0; width]; height];
            Canvas {
                pixels,
                width,
                height,
                _stride: width,
            }
        }

        pub fn fill(&mut self, color: u32) {
            self.pixels.iter_mut().flatten().for_each(|p| *p = color);
        }

        pub fn line(&mut self, color: u32, x1: i64, y1: i64, x2: i64, y2: i64) {
            let x1 = x1.clamp(0, self.width as i64 - 1);
            let x2 = x2.clamp(0, self.width as i64 - 1);
            let y1 = y1.clamp(0, self.height as i64 - 1);
            let y2 = y2.clamp(0, self.height as i64 - 1);

            // https://en.wikipedia.org/wiki/Bresenham%27s_line_algorithm
            let dx = (x2 - x1).abs();
            let dy = -(y2 - y1).abs();
            let sx = (x2 - x1).signum();
            let sy = (y2 - y1).signum();
            let mut error = dx + dy;
            let (mut x, mut y) = (x1, y1);

            while x != x2 || y != y2 {
                self.pixels[y as usize][x as usize] = color;
                let e = 2 * error;
                if e >= dy {
                    if x == x2 {
                        break;
                    }
                    error += dy;
                    x += sx;
                }
                if e <= dx {
                    if y == y2 {
                        break;
                    }
                    error += dx;
                    y += sy;
                }
            }
        }

        pub fn fill_rect(&mut self, color: u32, x0: i64, y0: i64, w: usize, h: usize) {
            let x1 = x0.clamp(0, self.width as i64 - 1) as usize;
            let y1 = y0.clamp(0, self.height as i64 - 1) as usize;
            let x2 = (x0 + w as i64).clamp(0, self.width as i64 - 1) as usize;
            let y2 = (y0 + h as i64).clamp(0, self.height as i64 - 1) as usize;

            for y in y1..y2 {
                for x in x1..x2 {
                    self.pixels[y][x] = color;
                }
            }
        }

        pub fn fill_circle(&mut self, color: u32, x0: i64, y0: i64, radius: usize) {
            let x1 = (x0 - radius as i64).clamp(0, self.width as i64 - 1) as usize;
            let y1 = (y0 - radius as i64).clamp(0, self.height as i64 - 1) as usize;
            let x2 = (x0 + radius as i64).clamp(0, self.width as i64 - 1) as usize;
            let y2 = (y0 + radius as i64).clamp(0, self.height as i64 - 1) as usize;

            for y in y1..y2 {
                for x in x1..x2 {
                    let dx = x as i64 - x0;
                    let dy = y as i64 - y0;
                    if dx * dx + dy * dy < (radius * radius) as i64 {
                        self.pixels[y][x] = color;
                    }
                }
            }
        }

        pub fn save_to_ppm(&self, file_path: &str) -> io::Result<()> {
            let mut file = BufWriter::new(File::create(file_path)?);
            file.write_fmt(format_args!("P6\n{} {} 255\n", self.width, self.height))?;

            for p in self.pixels.iter().flatten() {
                let bytes = [
                    ((p >> (8 * 0)) & 0xFF) as u8,
                    ((p >> (8 * 1)) & 0xFF) as u8,
                    ((p >> (8 * 2)) & 0xFF) as u8,
                ];
                file.write(&bytes[..])?;
            }

            Ok(())
        }
    }
}
