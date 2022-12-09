pub type Point = (usize, usize);
pub type TreeIterator<'a> = Box<dyn Iterator<Item = u8> + 'a>;
