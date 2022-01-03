extern crate peg;
use crate::amphipod::Amphipod;
use crate::amphipod_mover::new_amphipod_mover;
use crate::amphipod_mover::AmphipodMover;

peg::parser! { grammar day23_parser() for str {
    rule amber() -> Amphipod
        = "A" { Amphipod::Amber }
    rule bronze() -> Amphipod
        = "B" { Amphipod::Bronze }
    rule copper() -> Amphipod
        = "C" { Amphipod::Copper }
    rule desert() -> Amphipod
        = "D" { Amphipod::Desert }
    rule amphipod() -> Amphipod
        = amphipod:(amber() / bronze() / copper() / desert()) { amphipod }
    rule top() -> ()
        = "#############" " " * "\n" "#...........#" " " * "\n" {}
    rule bottom() -> ()
        = "  #########"
    pub rule parse() -> AmphipodMover
        = top() "###" a2:amphipod() "#" b2:amphipod() "#" c2:amphipod() "#" d2:amphipod() "###" " " * "\n"
        "  #" a1:amphipod() "#" b1:amphipod() "#" c1:amphipod() "#" d1:amphipod() "#" " " * "\n" bottom()  "\n" * {
            new_amphipod_mover(
                 vec![Some(a1), Some(a2)],
                 vec![Some(b1), Some(b2)],
                 vec![Some(c1), Some(c2)],
                 vec![Some(d1), Some(d2)],
                )
        }
}}

impl AmphipodMover {
    /// Generates a new AmphipodMover object to be calculated upon
    ///
    /// Inputs: the full string contents of the input data set.
    ///
    /// Returns: A new AmphipodMover object, with methods `calculate_day_a` and `calculate_day_b`
    /// available
    pub fn new(input_str: &str) -> AmphipodMover {
        day23_parser::parse(input_str).unwrap()
    }
}
