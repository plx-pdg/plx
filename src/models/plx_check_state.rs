use super::plx_check::PlxCheck;

pub struct PlxCheckState<'a> {
    check: &'a PlxCheck,
    passed: bool,
}
