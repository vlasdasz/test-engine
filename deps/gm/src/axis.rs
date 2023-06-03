use std::marker::ConstParamTy;

#[derive(Eq, PartialEq, Copy, ConstParamTy, Clone)]
pub enum Axis {
    X,
    Y,
}
