use super::check::Check;

pub struct CheckState<'a> {
    check: &'a Check,
    passed: bool,
}
