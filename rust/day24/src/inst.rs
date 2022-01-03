#[derive(Debug)]
pub enum Reg {
    W,
    X,
    Y,
    Z,
    Value(isize),
}

#[derive(Debug)]
pub enum Inst {
    Inp(Reg),
    Add(Reg, Reg),
    Mul(Reg, Reg),
    Div(Reg, Reg),
    Mod(Reg, Reg),
    Eql(Reg, Reg),
}
