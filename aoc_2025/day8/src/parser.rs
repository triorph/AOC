extern crate peg;
use aoc_helpers::point3d::Point3D;
use aoc_helpers::AOCFileOrParseError;

peg::parser! { pub grammar day8_parser() for str {
    rule number() -> usize
        = n:$(['0'..='9']+) { n.parse().unwrap_or_else(|_| panic!("Was expecting a number string {}", n))}
    rule location() -> Point3D
        = x:number() "," y:number() "," z:number() { Point3D::from_usize(x, y, z) }
    pub rule parse() -> Vec<Point3D>
        = lines:location() ++ ("\n" +) "\n" * { lines }
}}

pub fn parse_data(input: &str) -> Result<Vec<Point3D>, AOCFileOrParseError> {
    if let Ok(ret) = day8_parser::parse(input) {
        Ok(ret)
    } else {
        Err(AOCFileOrParseError)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    use aoc_helpers::read_input_file;
    #[test]
    fn test_parse() {
        let input_str = read_input_file("data/test_data.txt").unwrap();
        let actual = day8_parser::parse(&input_str).expect("Should parse successfully");
        let expected: Vec<Point3D> = vec![
            Point3D {
                x: 162,
                y: 817,
                z: 812,
            },
            Point3D {
                x: 57,
                y: 618,
                z: 57,
            },
            Point3D {
                x: 906,
                y: 360,
                z: 560,
            },
            Point3D {
                x: 592,
                y: 479,
                z: 940,
            },
            Point3D {
                x: 352,
                y: 342,
                z: 300,
            },
            Point3D {
                x: 466,
                y: 668,
                z: 158,
            },
            Point3D {
                x: 542,
                y: 29,
                z: 236,
            },
            Point3D {
                x: 431,
                y: 825,
                z: 988,
            },
            Point3D {
                x: 739,
                y: 650,
                z: 466,
            },
            Point3D {
                x: 52,
                y: 470,
                z: 668,
            },
            Point3D {
                x: 216,
                y: 146,
                z: 977,
            },
            Point3D {
                x: 819,
                y: 987,
                z: 18,
            },
            Point3D {
                x: 117,
                y: 168,
                z: 530,
            },
            Point3D {
                x: 805,
                y: 96,
                z: 715,
            },
            Point3D {
                x: 346,
                y: 949,
                z: 466,
            },
            Point3D {
                x: 970,
                y: 615,
                z: 88,
            },
            Point3D {
                x: 941,
                y: 993,
                z: 340,
            },
            Point3D {
                x: 862,
                y: 61,
                z: 35,
            },
            Point3D {
                x: 984,
                y: 92,
                z: 344,
            },
            Point3D {
                x: 425,
                y: 690,
                z: 689,
            },
        ];
        assert_eq!(expected, actual)
    }
}
