#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Check {
    Output { expected: String },
}
