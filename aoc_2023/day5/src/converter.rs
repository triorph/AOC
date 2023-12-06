pub struct ConverterMap {
    input_name: String,
    output_name: String,
    converters: Vec<Converter>,
}

#[derive(Debug)]
pub struct Converter {
    input_range: Range,
    output_start: u64,
}

pub type Range = (u64, u64);

trait RangeTrait {
    fn overlaps(&self, other: &Range) -> bool;
    fn contains(&self, other: &u64) -> bool;
    fn len(&self) -> u64;
}

impl RangeTrait for Range {
    fn overlaps(&self, other: &Range) -> bool {
        !(self.0 > other.1) && !(other.0 > self.1) && !(self.1 < other.0) && !(other.1 < self.0)
    }

    fn contains(&self, other: &u64) -> bool {
        (self.0..self.1).contains(other)
    }

    fn len(&self) -> u64 {
        self.1 - self.0
    }
}

impl Converter {
    pub fn new(output_start: u64, input_start: u64, range: u64) -> Converter {
        Converter {
            input_range: (input_start, input_start + range),
            output_start,
        }
    }

    fn convert(&self, input: u64) -> Option<u64> {
        if self.input_range.contains(&input) {
            return Some(self.output_start + input - self.input_range.0);
        }
        None
    }

    fn convert_range(&self, other_range: Range) -> (Vec<Range>, Vec<Range>) {
        let mut changed = vec![];
        let mut unchanged = vec![];
        if self.input_range.overlaps(&other_range) {
            let start = if self.input_range.contains(&other_range.0) {
                self.output_start + other_range.0 - self.input_range.0
            } else {
                if self.input_range.0 > other_range.0 {
                    unchanged.push((other_range.0, self.input_range.0));
                }
                self.output_start
            };
            let end = if self.input_range.contains(&other_range.1) {
                self.output_start + other_range.1 - self.input_range.0
            } else {
                if other_range.1 > self.input_range.1 {
                    unchanged.push((self.input_range.1, other_range.1));
                }
                self.output_start + self.input_range.len()
            };
            changed.push((start, end))
        } else {
            unchanged.push(other_range);
        }
        (changed, unchanged)
    }
}

impl ConverterMap {
    pub fn new(input: &str, output: &str, converters: Vec<Converter>) -> ConverterMap {
        ConverterMap {
            input_name: input.to_string(),
            output_name: output.to_string(),
            converters,
        }
    }

    pub fn convert(&self, input: u64) -> u64 {
        for converter in self.converters.iter() {
            if let Some(result) = converter.convert(input) {
                return result;
            }
        }
        input
    }

    pub fn convert_range(&self, range: &Range) -> Vec<Range> {
        let mut unmapped: Vec<Range> = vec![range.clone()];
        let mut results: Vec<Range> = vec![];
        for converter in self.converters.iter() {
            let mut next_unmapped = vec![];
            for range in unmapped.iter() {
                let (changed, unchanged) = converter.convert_range((range.0, range.1));
                // println!(
                //     "Converter {:?} converted {:?} to {:?} and left {:?}",
                //     converter, range, changed, unchanged
                // );
                results.extend(changed);
                next_unmapped.extend(unchanged);
            }
            unmapped = next_unmapped
        }
        results.extend(unmapped);
        results
    }

    pub fn convert_ranges(&self, input: &[Range]) -> Vec<Range> {
        println!(
            "\n\nConverter {:?}, ranges initial input: {:?}\n\n",
            (&self.input_name, &self.output_name),
            input
        );
        let mut ret = vec![];
        for range in input.iter() {
            // println!("\n input range: {:?}\n", range);
            let interim = self.convert_range(range);
            // println!("\n was converted to: {:?}\n", interim);
            ret.extend(interim);
        }
        println!("\n\nConverter ranges final result: {:?}\n\n", ret);
        ret
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::{Converter, ConverterMap};

    #[test]
    fn test_below() {
        let converter_map = ConverterMap::new("a", "b", vec![Converter::new(70, 50, 10)]);
        let actual = converter_map.convert_ranges(&[(45, 55)]);
        let expected = vec![(70, 75), (45, 50)];
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_above() {
        let converter_map = ConverterMap::new("a", "b", vec![Converter::new(70, 50, 10)]);
        let actual = converter_map.convert_ranges(&[(55, 65)]);
        let expected = vec![(75, 80), (60, 65)];
        assert_eq!(actual, expected)
    }

    #[test]
    fn test_within() {
        let converter_map = ConverterMap::new("a", "b", vec![Converter::new(70, 50, 15)]);
        let actual = converter_map.convert_ranges(&[(55, 60)]);
        let expected = vec![(75, 80)];
        assert_eq!(actual, expected)
    }

    #[test]
    fn test_around() {
        let converter_map = ConverterMap::new("a", "b", vec![Converter::new(70, 50, 5)]);
        let actual = converter_map.convert_ranges(&[(45, 60)]);
        let expected = vec![(70, 75), (45, 50), (55, 60)];
        assert_eq!(actual, expected)
    }
}
