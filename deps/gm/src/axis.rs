use std::marker::ConstParamTy;

#[derive(Copy, Clone, PartialEq, Eq, ConstParamTy)]
pub enum Axis {
    X,
    Y,
}
