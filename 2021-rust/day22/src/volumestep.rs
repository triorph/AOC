use crate::cuboid::Cuboid;
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct VolumeStep {
    pub volume_type: bool,
    pub block: Cuboid,
}
