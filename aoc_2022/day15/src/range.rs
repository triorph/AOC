#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct Range {
    pub start: isize,
    pub end: isize,
}

#[derive(Debug)]
pub struct RangeCollection {
    pub ranges: Vec<Range>,
}

impl Range {
    pub fn does_overlap(&self, other: &Range) -> bool {
        self.end >= other.start && self.start <= other.end
    }

    pub fn merge(&self, other: &Range) -> Range {
        // Should only use if does_overlap = True
        Range {
            start: self.start.min(other.start),
            end: self.end.max(other.end),
        }
    }

    fn size(&self) -> usize {
        (self.end - self.start) as usize
    }
}

impl RangeCollection {
    pub fn new() -> RangeCollection {
        RangeCollection { ranges: Vec::new() }
    }
    pub fn get_size(&self) -> usize {
        self.ranges.iter().map(|range| range.size()).sum::<usize>()
    }

    pub fn add(&mut self, start: isize, end: isize) {
        let new_range = Range { start, end };
        let mut overlapped = Vec::new();
        for i in 0..self.ranges.len() {
            if self.ranges[i].does_overlap(&new_range) {
                self.ranges[i] = self.ranges[i].merge(&new_range);
                overlapped.push(i);
            }
        }
        if overlapped.is_empty() {
            self.ranges.push(new_range);
        } else if overlapped.len() > 1 {
            self.merge_overlaps(&overlapped)
        }
    }

    fn merge_overlaps(&mut self, indices: &[usize]) {
        // First merge all entries that overlapped the new one
        let overlapped = indices
            .iter()
            .map(|i| self.ranges[*i])
            .reduce(|overlapped, next| overlapped.merge(&next))
            .unwrap();
        // Second remove said entries from the list of full ranges
        self.ranges = self
            .ranges
            .iter()
            .enumerate()
            .filter(|(i, _)| !indices.contains(i))
            .map(|(_, range)| range)
            .copied()
            .collect();
        // Lastly add the newly merged overlapped entry to the list
        self.ranges.push(overlapped);
    }
}
