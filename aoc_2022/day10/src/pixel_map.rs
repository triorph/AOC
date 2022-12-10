#[derive(Debug, PartialEq)]
pub struct PixelMap {
    pixels: [bool; 40 * 6],
    cycle: usize,
}

impl PixelMap {
    pub fn new() -> PixelMap {
        PixelMap {
            pixels: [false; 40 * 6],
            cycle: 0,
        }
    }

    pub fn draw_current(&mut self, x: isize) {
        let x_pos = self.cycle % 40;
        if ((x - 1)..=(x + 1)).contains(&(x_pos as isize)) {
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

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_pixel_result() {
        let mut expected: String = "".into();
        expected += "##..##..##..##..##..##..##..##..##..##..";
        expected += "###...###...###...###...###...###...###.";
        expected += "####....####....####....####....####....";
        expected += "#####.....#####.....#####.....#####.....";
        expected += "######......######......######......####";
        expected += "#######.......#######.......#######.....";
    }
}
