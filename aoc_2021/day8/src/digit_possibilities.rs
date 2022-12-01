use crate::digit_segment::DigitSegment;

pub struct DigitPossibilities([Vec<DigitSegment>; 7]);

impl DigitPossibilities {
    pub fn new() -> DigitPossibilities {
        DigitPossibilities([
            DigitSegment::all_segments().to_vec(),
            DigitSegment::all_segments().to_vec(),
            DigitSegment::all_segments().to_vec(),
            DigitSegment::all_segments().to_vec(),
            DigitSegment::all_segments().to_vec(),
            DigitSegment::all_segments().to_vec(),
            DigitSegment::all_segments().to_vec(),
        ])
    }

    fn index_for_key(key: &DigitSegment) -> usize {
        match *key {
            DigitSegment::A => 0,
            DigitSegment::B => 1,
            DigitSegment::C => 2,
            DigitSegment::D => 3,
            DigitSegment::E => 4,
            DigitSegment::F => 5,
            DigitSegment::G => 6,
        }
    }

    pub fn get(self: &DigitPossibilities, key: &DigitSegment) -> &Vec<DigitSegment> {
        &self.0[DigitPossibilities::index_for_key(key)]
    }

    fn set(self: &mut DigitPossibilities, key: &DigitSegment, value: Vec<DigitSegment>) {
        self.0[DigitPossibilities::index_for_key(key)] = value;
    }

    pub fn filter_down_possibilities(
        self: &mut DigitPossibilities,
        inputs: &[DigitSegment],
        outputs: &[DigitSegment],
    ) {
        // A little confusing. Tries to work out if any segments
        // have enough knowledge of their possible mappings to
        // infer new knowledge for remaining segments.
        if inputs.len() == outputs.len() {
            for digit_segment in DigitSegment::all_segments().iter() {
                let old_outputs_for_input = self.get(digit_segment);
                let new_outputs_for_input = old_outputs_for_input
                    .iter()
                    .filter(|possible_value| {
                        if inputs.contains(digit_segment) {
                            outputs.contains(possible_value)
                        } else {
                            !outputs.contains(possible_value)
                        }
                    })
                    .copied()
                    .collect::<Vec<DigitSegment>>();
                self.set(digit_segment, new_outputs_for_input);
            }
        }
    }

    pub fn build_possible_outputs(
        self: &DigitPossibilities,
        inputs: &[DigitSegment],
    ) -> Vec<DigitSegment> {
        // Calculates which segment outputs are possible for a given input
        // segment choice (given what we have currently worked out)
        let mut possible_outputs = vec![];
        for digit_segment in &inputs[..] {
            for output_digit_segment in &self.get(digit_segment)[..] {
                if !possible_outputs.contains(output_digit_segment) {
                    possible_outputs.push(*output_digit_segment);
                }
            }
        }
        possible_outputs
    }
}
