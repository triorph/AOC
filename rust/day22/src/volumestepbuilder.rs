use crate::cuboid::Cuboid;
use crate::volumestep::VolumeStep;

pub struct VolumeStepBuilder {
    pub volume_steps: Vec<VolumeStep>,
}

impl VolumeStepBuilder {
    fn subtract_from_other(to_be_subtracted: &[Cuboid], to_subtract: &[Cuboid]) -> Vec<Cuboid> {
        to_be_subtracted
            .iter()
            .map(|vol_to_be| vol_to_be.repeatedly_subtract_from(to_subtract))
            .flatten()
            .collect::<Vec<Cuboid>>()
    }

    fn get_next_volume_step(&mut self) -> VolumeStep {
        let ret = self.volume_steps[0].clone();
        self.volume_steps = self.volume_steps[1..].to_vec();
        ret
    }

    fn run_one_step(&mut self, resulting_volumes: &[Cuboid], check_day_a: bool) -> Vec<Cuboid> {
        let step = self.get_next_volume_step();
        if check_day_a && step.block.outside_day_a_limit() {
            return resulting_volumes.to_vec();
        }
        let mut resulting_volumes =
            VolumeStepBuilder::subtract_from_other(resulting_volumes, &[step.block.clone()]);
        if step.volume_type {
            resulting_volumes.push(step.block);
        }
        resulting_volumes
    }

    fn calculate_all_volumes_size(volumes: &[Cuboid]) -> usize {
        volumes.iter().map(|v| v.calculate_volume()).sum::<usize>()
    }

    fn run_steps(&mut self, check_day_a: bool) -> Vec<Cuboid> {
        let mut resulting_volumes = vec![];
        while !self.volume_steps.is_empty() {
            resulting_volumes = self.run_one_step(&resulting_volumes, check_day_a);
        }
        resulting_volumes
    }

    /// Calculate the part a response
    pub fn calculate_day_a(&mut self) -> usize {
        let volumes = self.run_steps(true);
        VolumeStepBuilder::calculate_all_volumes_size(&volumes)
    }

    /// Calculate the part b response
    pub fn calculate_day_b(&mut self) -> usize {
        let volumes = self.run_steps(false);
        VolumeStepBuilder::calculate_all_volumes_size(&volumes)
    }
}

#[cfg(test)]
mod test {
    use crate::cuboid::Cuboid;
    use crate::volumestepbuilder::VolumeStepBuilder;

    #[test]
    fn test_parse() {
        let _day22_setup = VolumeStepBuilder::new(include_str!("../test_data.txt"));
    }

    #[test]
    fn test_subtract_single() {
        let volume_a = Cuboid::new(1, 1, 1, 3, 1, 3);
        let volume_b = Cuboid::new(1, 3, 1, 3, 1, 3);
        let subtracted_volumes = VolumeStepBuilder::subtract_from_other(&[volume_b], &[volume_a]);
        let new_volume: usize = subtracted_volumes
            .iter()
            .map(|v| v.calculate_volume())
            .sum();
        assert_eq!(new_volume, 18);
    }

    #[test]
    fn test_subtract_multiple() {
        // we have 2 volumes with a gap of 1 between, and are trying to add a volume
        // covering all 3.
        //
        // We should subtract both existing volumes from the new volume before adding, and
        // end up with the new size.
        let volume_a = Cuboid::new(1, 1, 1, 3, 1, 3);
        let volume_b = Cuboid::new(3, 3, 1, 3, 1, 3);
        let volume_c = Cuboid::new(1, 3, 1, 3, 1, 3);
        let subtracted_volumes =
            VolumeStepBuilder::subtract_from_other(&[volume_c], &[volume_a, volume_b]);
        let new_volume: usize = subtracted_volumes
            .iter()
            .map(|v| v.calculate_volume())
            .sum();
        assert_eq!(new_volume, 9);
    }

    #[test]
    fn test_day_a() {
        let mut day22_setup = VolumeStepBuilder::new(include_str!("../test_data.txt"));
        assert_eq!(day22_setup.calculate_day_a(), 39);
    }

    #[test]
    fn test_daya_larger() {
        let mut day22_setup = VolumeStepBuilder::new(include_str!("../test_data2.txt"));
        assert_eq!(day22_setup.calculate_day_a(), 590784);
    }

    #[test]
    fn test_daya_larger_step_by_step() {
        let mut day22_setup = VolumeStepBuilder::new(include_str!("../test_data2.txt"));
        let next = day22_setup.run_one_step(&[], true);
        assert_eq!(VolumeStepBuilder::calculate_all_volumes_size(&next), 139590);
        let next = day22_setup.run_one_step(&next, true);
        assert_eq!(VolumeStepBuilder::calculate_all_volumes_size(&next), 210918);
        let next = day22_setup.run_one_step(&next, true);
        assert_eq!(VolumeStepBuilder::calculate_all_volumes_size(&next), 225476);
        let next = day22_setup.run_one_step(&next, true);
        assert_eq!(VolumeStepBuilder::calculate_all_volumes_size(&next), 328328);
        let next = day22_setup.run_one_step(&next, true);
        assert_eq!(VolumeStepBuilder::calculate_all_volumes_size(&next), 387734);
        let next = day22_setup.run_one_step(&next, true);
        assert_eq!(VolumeStepBuilder::calculate_all_volumes_size(&next), 420416);
        let next = day22_setup.run_one_step(&next, true);
        assert_eq!(VolumeStepBuilder::calculate_all_volumes_size(&next), 436132);
        let next = day22_setup.run_one_step(&next, true);
        assert_eq!(VolumeStepBuilder::calculate_all_volumes_size(&next), 478727);
        let next = day22_setup.run_one_step(&next, true);
        assert_eq!(VolumeStepBuilder::calculate_all_volumes_size(&next), 494759);
        let next = day22_setup.run_one_step(&next, true);
        assert_eq!(VolumeStepBuilder::calculate_all_volumes_size(&next), 494804);
        let next = day22_setup.run_one_step(&next, true);
        assert_eq!(VolumeStepBuilder::calculate_all_volumes_size(&next), 492164);
        let next = day22_setup.run_one_step(&next, true);
        assert_eq!(VolumeStepBuilder::calculate_all_volumes_size(&next), 534936);
        let next = day22_setup.run_one_step(&next, true);
        assert_eq!(VolumeStepBuilder::calculate_all_volumes_size(&next), 534936);
        let next = day22_setup.run_one_step(&next, true);
        assert_eq!(VolumeStepBuilder::calculate_all_volumes_size(&next), 567192,);
        let next = day22_setup.run_one_step(&next, true);
        assert_eq!(VolumeStepBuilder::calculate_all_volumes_size(&next), 567150);
        let next = day22_setup.run_one_step(&next, true);
        assert_eq!(VolumeStepBuilder::calculate_all_volumes_size(&next), 592167);
        let next = day22_setup.run_one_step(&next, true);
        assert_eq!(VolumeStepBuilder::calculate_all_volumes_size(&next), 588567);
        let next = day22_setup.run_one_step(&next, true);
        assert_eq!(VolumeStepBuilder::calculate_all_volumes_size(&next), 592902);
        let next = day22_setup.run_one_step(&next, true);
        assert_eq!(VolumeStepBuilder::calculate_all_volumes_size(&next), 590029);
        let next = day22_setup.run_one_step(&next, true);
        assert_eq!(VolumeStepBuilder::calculate_all_volumes_size(&next), 590784);
    }

    #[test]
    fn test_day_a_with_b_input() {
        let mut day22_setup = VolumeStepBuilder::new(include_str!("../test_data3.txt"));
        assert_eq!(day22_setup.calculate_day_a(), 474140);
    }

    #[test]
    fn test_day_b() {
        let mut day22_setup = VolumeStepBuilder::new(include_str!("../test_data3.txt"));
        assert_eq!(day22_setup.calculate_day_b(), 2758514936282235);
    }
}
