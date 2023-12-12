use std::ops::Range;

pub struct ConverterMap {
    converters: Vec<Converter>,
}

#[derive(Debug)]
pub struct Converter {
    input_range: Range<usize>,
    output_start: usize,
}

trait Overlaps {
    fn overlaps(&self, other: &Self) -> bool;
}

impl Overlaps for Range<usize> {
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

    fn output_end(&self) -> usize {
        self.output_start + self.input_range.len()
    }

    fn convert_range(&self, other_range: Range<usize>) -> (Vec<Range<usize>>, Vec<Range<usize>>) {
        let mut changed = vec![];
        let mut unchanged = vec![];
        if self.input_range.overlaps(&other_range) {
            let start = self.convert(other_range.start).unwrap_or_else(|| {
                unchanged.push(other_range.start..self.input_range.start);
                self.output_start
            });
            let end = self.convert(other_range.end).unwrap_or_else(|| {
                unchanged.push(self.input_range.end..other_range.end);
                self.output_end()
            });
            changed.push(start..end)
        } else {
            unchanged.push(other_range);
        }
        (changed, unchanged)
    }

    fn convert_ranges(&self, ranges: &[Range<usize>]) -> (Vec<Range<usize>>, Vec<Range<usize>>) {
        ranges
            .iter()
            .map(|range| self.convert_range(range.clone()))
            .fold(
                (Vec::new(), Vec::new()),
                |(changed, unchanged), (new_changed, new_unchanged)| {
                    (
                        [changed, new_changed].concat(),
                        [unchanged, new_unchanged].concat(),
                    )
                },
            )
    }
}

impl ConverterMap {
    pub fn new(converters: Vec<Converter>) -> ConverterMap {
        ConverterMap { converters }
    }

    pub fn convert(&self, input: usize) -> usize {
        self.converters
            .iter()
            .map(|converter| converter.convert(input))
            .filter(|result| result.is_some())
            .flatten()
            .next()
            .unwrap_or(input)
    }

    pub fn convert_range(&self, range: &Range<usize>) -> Vec<Range<usize>> {
        let (results, unchanged) = self.converters.iter().fold(
            (vec![range.clone()], Vec::new()),
            |(to_convert, results), converter| {
                let (changed, unchanged) = converter.convert_ranges(&to_convert);
                (unchanged, [results, changed].concat())
            },
        );
        [unchanged, results].concat()
    }

    pub fn convert_ranges(&self, input: &[Range<usize>]) -> Vec<Range<usize>> {
        input
            .iter()
            .flat_map(|range| self.convert_range(range))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::{Converter, ConverterMap};

    #[test]
    #[allow(clippy::single_range_in_vec_init)]
    fn test_below() {
        let converter_map = ConverterMap::new(vec![Converter::new(70, 50, 10)]);
        let actual = converter_map.convert_ranges(&[(45..55)]);
        let expected = vec![(70..75), (45..50)];
        assert_eq!(actual, expected);
    }

    #[test]
    #[allow(clippy::single_range_in_vec_init)]
    fn test_above() {
        let converter_map = ConverterMap::new(vec![Converter::new(70, 50, 10)]);
        let actual = converter_map.convert_ranges(&[(55..65)]);
        let expected = vec![(75..80), (60..65)];
        assert_eq!(actual, expected)
    }

    #[test]
    #[allow(clippy::single_range_in_vec_init)]
    fn test_within() {
        let converter_map = ConverterMap::new(vec![Converter::new(70, 50, 15)]);
        let actual = converter_map.convert_ranges(&[(55..60)]);
        let expected = vec![(75..80)];
        assert_eq!(actual, expected)
    }

    #[test]
    #[allow(clippy::single_range_in_vec_init)]
    fn test_around() {
        let converter_map = ConverterMap::new(vec![Converter::new(70, 50, 5)]);
        let actual = converter_map.convert_ranges(&[(45..60)]);
        let expected = vec![(70..75), (45..50), (55..60)];
        assert_eq!(actual, expected)
    }
}
