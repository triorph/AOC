use std::ops::Range;

pub struct ConverterMap {
    converters: Vec<Converter>,
}

#[derive(Debug)]
pub struct Converter {
    input_range: Range<usize>,
    output_start: usize,
}

pub trait RangeTrait {
    fn overlaps(&self, other: &Self) -> bool;
}

impl RangeTrait for Range<usize> {
    fn overlaps(&self, other: &Self) -> bool {
        !(self.start > other.end || other.start > self.end)
            && self.end >= other.start
            && other.end >= self.start
    }
}

impl Converter {
    pub fn new(output_start: usize, input_start: usize, range: usize) -> Converter {
        Converter {
            input_range: input_start..input_start + range,
            output_start,
        }
    }

    fn convert(&self, input: usize) -> Option<usize> {
        if self.input_range.contains(&input) {
            return Some(self.output_start + input - self.input_range.start);
        }
        None
    }

    fn convert_range(&self, other_range: Range<usize>) -> (Vec<Range<usize>>, Vec<Range<usize>>) {
        let mut changed = vec![];
        let mut unchanged = vec![];
        if self.input_range.overlaps(&other_range) {
            let start = if self.input_range.contains(&other_range.start) {
                self.output_start + other_range.start - self.input_range.start
            } else {
                if self.input_range.start > other_range.start {
                    unchanged.push(other_range.start..self.input_range.start);
                }
                self.output_start
            };
            let end = if self.input_range.contains(&other_range.end) {
                self.output_start + other_range.end - self.input_range.start
            } else {
                if other_range.end > self.input_range.end {
                    unchanged.push(self.input_range.end..other_range.end);
                }
                self.output_start + self.input_range.len()
            };
            changed.push(start..end)
        } else {
            unchanged.push(other_range);
        }
        (changed, unchanged)
    }
}

impl ConverterMap {
    pub fn new(converters: Vec<Converter>) -> ConverterMap {
        ConverterMap { converters }
    }

    pub fn convert(&self, input: usize) -> usize {
        for converter in self.converters.iter() {
            if let Some(result) = converter.convert(input) {
                return result;
            }
        }
        input
    }

    pub fn convert_range(&self, range: &Range<usize>) -> Vec<Range<usize>> {
        let mut unmapped: Vec<Range<usize>> = vec![range.clone()];
        let mut results: Vec<Range<usize>> = vec![];
        for converter in self.converters.iter() {
            let mut next_unmapped = vec![];
            for range in unmapped.iter() {
                let (changed, unchanged) = converter.convert_range(range.clone());
                results.extend(changed);
                next_unmapped.extend(unchanged);
            }
            unmapped = next_unmapped
        }
        results.extend(unmapped);
        results
    }

    pub fn convert_ranges(&self, input: &[Range<usize>]) -> Vec<Range<usize>> {
        let mut ret = vec![];
        for range in input.iter() {
            let interim = self.convert_range(range);
            ret.extend(interim);
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::{Converter, ConverterMap};

    #[test]
    fn test_below() {
        let converter_map = ConverterMap::new(vec![Converter::new(70, 50, 10)]);
        let actual = converter_map.convert_ranges(&[(45..55)]);
        let expected = vec![(70..75), (45..50)];
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_above() {
        let converter_map = ConverterMap::new(vec![Converter::new(70, 50, 10)]);
        let actual = converter_map.convert_ranges(&[(55..65)]);
        let expected = vec![(75..80), (60..65)];
        assert_eq!(actual, expected)
    }

    #[test]
    fn test_within() {
        let converter_map = ConverterMap::new(vec![Converter::new(70, 50, 15)]);
        let actual = converter_map.convert_ranges(&[(55..60)]);
        let expected = vec![(75..80)];
        assert_eq!(actual, expected)
    }

    #[test]
    fn test_around() {
        let converter_map = ConverterMap::new(vec![Converter::new(70, 50, 5)]);
        let actual = converter_map.convert_ranges(&[(45..60)]);
        let expected = vec![(70..75), (45..50), (55..60)];
        assert_eq!(actual, expected)
    }
}
