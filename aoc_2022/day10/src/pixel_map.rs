#[derive(Debug, Clone)]
pub struct PixelMap {
    pixels: [bool; 40 * 6],
    cycle: usize,
    x: isize,
    pub day_a_result: usize,
}

impl PixelMap {
    pub fn new() -> PixelMap {
        PixelMap {
            pixels: [false; 40 * 6],
            cycle: 0,
            x: 1,
            day_a_result: 0,
        }
    }

    pub fn draw_next_pixel(&mut self) {
        let x_pos = self.cycle % 40;
        if ((self.x - 1)..=(self.x + 1)).contains(&(x_pos as isize)) {
            self.pixels[self.cycle] = true;
        }
    }

    fn increase_cycle(&mut self) {
        self.draw_next_pixel();
        self.cycle += 1;
        if self.cycle % 40 == 20 {
            self.day_a_result += self.x as usize * self.cycle;
        }
    }

    pub fn process_instruction(&mut self, instruction: &Option<isize>) {
        match instruction {
            Some(x) => {
                self.increase_cycle();
                self.increase_cycle();
                self.x += x;
            }
            None => {
                self.increase_cycle();
            }
        }
    }
}

impl std::fmt::Display for PixelMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut ret = "".to_string();
        for i in 0..6 {
            for j in 0..40 {
                ret += if self.pixels[i * 40 + j] { "█" } else { " " };
            }
            ret += "\n";
        }
        write!(f, "{}", ret)
    }
}
