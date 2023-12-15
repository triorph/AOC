use crate::step::{AOCHash, Operation, Step};

pub type LensBoxType = Vec<(String, usize)>;

pub trait LensBox {
    fn add_step_to_box(&self, label: &str, value: usize) -> Self;
    fn remove_step_from_box(&self, label: &str) -> Self;
}

impl LensBox for LensBoxType {
    fn add_step_to_box(&self, label: &str, value: usize) -> Self {
        let label_index = self
            .iter()
            .enumerate()
            .find(|(_, (element_label, _))| element_label == label);
        let mut ret = self.clone();
        if let Some((label_index, _)) = label_index {
            ret[label_index].1 = value;
        } else {
            ret.push((label.to_string(), value));
        }
        ret
    }

    fn remove_step_from_box(&self, label: &str) -> Self {
        self.iter()
            .filter(|(element_label, _)| element_label != label)
            .cloned()
            .collect()
    }
}

pub type LensBoxesType = [LensBoxType; 256];

pub trait LensBoxes {
    fn get_focusing_power(&self) -> usize;
    fn new() -> Self;
    fn process_steps(&self, steps: &[Step]) -> Self;
    fn process_step(&self, step: &Step) -> Self;
}

impl LensBoxes for LensBoxesType {
    fn new() -> Self {
        const STARTER: LensBoxType = Vec::new();
        [STARTER; 256]
    }

    fn get_focusing_power(&self) -> usize {
        self.iter()
            .enumerate()
            .map(|(i, lens_box)| {
                lens_box
                    .iter()
                    .enumerate()
                    .map(|(j, (_, focal_lens))| (i + 1) * (j + 1) * focal_lens)
                    .sum::<usize>()
            })
            .sum()
    }

    fn process_step(&self, step: &Step) -> Self {
        let mut ret = self.to_owned();
        let box_index = step.label.get_aoc_hash();
        match step.operation {
            Operation::Equals(value) => {
                ret[box_index] = self[box_index].add_step_to_box(&step.label, value);
            }
            Operation::Subtract => {
                ret[box_index] = self[box_index].remove_step_from_box(&step.label)
            }
        }
        ret
    }

    fn process_steps(&self, steps: &[Step]) -> Self {
        steps
            .iter()
            .fold(self.to_owned(), |lens_boxes: LensBoxesType, step: &Step| {
                lens_boxes.process_step(step)
            })
    }
}
