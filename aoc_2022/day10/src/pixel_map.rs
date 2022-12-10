#[derive(Debug, Clone)]
pub struct PixelMap {
    pixels: [bool; 40 * 6],
    pub cycle: usize,
}

impl PixelMap {
    pub fn new() -> PixelMap {
        PixelMap {
            pixels: [false; 40 * 6],
            cycle: 0,
        }
    }

    pub fn draw_next_pixel(&mut self, sprite: isize) {
        let x_pos = self.cycle % 40;
        if ((sprite - 1)..=(sprite + 1)).contains(&(x_pos as isize)) {
            self.pixels[self.cycle] = true;
        }

        self.cycle += 1;
    }
}

impl std::fmt::Display for PixelMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut ret = "".to_string();
        for i in 0..6 {
            for j in 0..40 {
                ret += if self.pixels[i * 40 + j] { "#" } else { "." };
            }
            ret += "\n";
        }
        write!(f, "{}", ret)
    }
}
