#[derive(Clone, Debug)]
pub enum MonadInst {
    Value(isize),
    Input(usize),
    Add(Box<MonadInst>, Box<MonadInst>),
    Mul(Box<MonadInst>, Box<MonadInst>),
}

impl MonadInst {
    pub fn is_zero(&self) -> bool {
        matches!(self, MonadInst::Value(0))
    }

    fn is_one(&self) -> bool {
        matches!(self, MonadInst::Value(1))
    }

    pub fn is_literal(&self) -> bool {
        matches!(self, MonadInst::Value(_))
    }

    fn get_literal_value(&self) -> isize {
        if let MonadInst::Value(val) = self {
            *val
        } else {
            panic!("Must call get_literal_value on a MonadInst::Value")
        }
    }

    pub fn literal_equal(&self, other: &MonadInst) -> MonadInst {
        match (self, other) {
            (MonadInst::Value(s), MonadInst::Value(o)) => {
                if s == o {
                    MonadInst::Value(1)
                } else {
                    MonadInst::Value(0)
                }
            }
            _ => MonadInst::Value(0),
        }
    }

    pub fn is_literal_invalid_for_input(&self) -> bool {
        if let MonadInst::Value(val) = self {
            !(1..=9).contains(val)
        } else {
            false
        }
    }

    pub fn is_input(&self) -> bool {
        matches!(self, MonadInst::Input(_))
    }

    fn is_add_pair(&self) -> bool {
        // Is A + C for a constant C.
        if let MonadInst::Add(_, right) = self {
            (**right).is_literal()
        } else {
            false
        }
    }

    fn get_add_pairs(&self) -> (MonadInst, MonadInst) {
        if let MonadInst::Add(left, right) = self {
            return ((**left).clone(), (**right).clone());
        }
        panic!("Called get_add_pairs on a non-add-pair")
    }

    fn is_26(&self) -> bool {
        matches!(self, MonadInst::Value(26))
    }

    fn is_26_tuple(&self) -> bool {
        // Is of the form A * 26 + B
        if let MonadInst::Add(left, _) = self {
            if let MonadInst::Mul(_, right2) = &**left {
                return (right2).is_26();
            }
        }
        false
    }

    fn get_left_26_tuple(&self) -> MonadInst {
        // for A * 26 + B, get A.
        if let MonadInst::Add(left, _) = self {
            if let MonadInst::Mul(left2, _) = &**left {
                return (**left2).clone();
            }
        }
        panic!("No left 26 tuple to get");
    }

    fn get_right_26_tuple(&self) -> MonadInst {
        // For A * 26 + B, get B.
        if let MonadInst::Add(_, right) = self {
            return (**right).clone();
        }
        panic!("No right 26 tuple to get");
    }

    pub fn get_input_operating_on(&self) -> usize {
        // MonadInst with an input looks either like
        // input(X) + C, or input(Y). We want to find X or Y.
        match self {
            MonadInst::Input(inp) => *inp,
            MonadInst::Add(left, right) => {
                if right.is_literal() {
                    if let MonadInst::Input(inp) = **left {
                        inp
                    } else {
                        panic!("Called get_input_operating_on on invalid MonadInst type")
                    }
                } else {
                    panic!("Called get_input_operating_on on invalid MonadInst type")
                }
            }
            _ => panic!("Called get_input_operating_on on invalid MonadInst type"),
        }
    }

    pub fn evaluate(&self, value: usize) -> isize {
        // Almost exclusively called on comparisons of Input(X) + C = Input(Y).
        //
        // In both cases, there's only 1 input, so we can provide it in advance if its needed.
        match self {
            MonadInst::Input(_) => value as isize,
            MonadInst::Value(val) => *val,
            MonadInst::Add(left, right) => left.evaluate(value) + right.evaluate(value),
            MonadInst::Mul(left, right) => left.evaluate(value) * right.evaluate(value),
        }
    }
}

impl std::ops::Add for &MonadInst {
    type Output = MonadInst;
    fn add(self, other: &MonadInst) -> MonadInst {
        if self.is_zero() {
            other.clone()
        } else if other.is_zero() {
            self.clone()
        } else if self.is_literal() && other.is_literal() {
            MonadInst::Value(self.get_literal_value() + other.get_literal_value())
        } else if other.is_literal() && self.is_add_pair() {
            // Turn X + C1 + C2 into X + C3 by summing the constants C1 and C2
            let (left, right) = self.get_add_pairs();
            MonadInst::Add(
                Box::new(left),
                Box::new(MonadInst::Value(
                    right.get_literal_value() + other.get_literal_value(),
                )),
            )
        } else {
            MonadInst::Add(Box::new(self.clone()), Box::new(other.clone()))
        }
    }
}

impl std::ops::Mul for &MonadInst {
    type Output = MonadInst;
    fn mul(self, other: &MonadInst) -> MonadInst {
        if self.is_zero() || other.is_zero() {
            MonadInst::Value(0)
        } else if self.is_one() {
            other.clone()
        } else if other.is_one() {
            self.clone()
        } else if self.is_literal() && other.is_literal() {
            MonadInst::Value(self.get_literal_value() * other.get_literal_value())
        } else {
            // Usually creating a 26_tuple.
            MonadInst::Mul(Box::new(self.clone()), Box::new(other.clone()))
        }
    }
}

impl std::ops::Div for &MonadInst {
    type Output = MonadInst;
    fn div(self, other: &MonadInst) -> MonadInst {
        // Almost exclusively calling / 26 to separate out 26_tuples, or /1 as a NOOP.
        if self.is_zero() {
            MonadInst::Value(0)
        } else if other.is_one() {
            self.clone()
        } else if self.is_literal() && other.is_literal() {
            MonadInst::Value(self.get_literal_value() / other.get_literal_value())
        } else if other.is_26() && self.is_26_tuple() {
            self.get_left_26_tuple()
        } else {
            // Naive case, that other is 26 and we are not a 26_tuple, so we get
            // reduced to 0.
            MonadInst::Value(0)
        }
    }
}

impl std::ops::Rem for &MonadInst {
    type Output = MonadInst;
    fn rem(self, other: &MonadInst) -> MonadInst {
        // Almost exclusively calling % 26 to separate out tuples.
        if self.is_zero() {
            MonadInst::Value(0)
        } else if self.is_literal() && other.is_literal() {
            MonadInst::Value(self.get_literal_value() % other.get_literal_value())
        } else if other.is_26() && self.is_26_tuple() {
            self.get_right_26_tuple()
        } else {
            // Naive case, that other is 26 and we are not a 26_tuple, so we just want ourself as a
            // whole.
            self.clone()
        }
    }
}
