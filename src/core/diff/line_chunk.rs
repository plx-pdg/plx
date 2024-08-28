use console::Style;

#[derive(Debug, PartialEq, Eq)]
pub(super) struct LineChunk {
    is_different: bool,
    value: String,
}

impl LineChunk {
    pub fn new(value: String, is_different: bool) -> Self {
        Self {
            value,
            is_different,
        }
    }
    pub(super) fn to_ansi_colors(&self, style: &Style) -> String {
        if self.is_different {
            format!("{}", style.apply_to(&self.value).bold())
        } else {
            format!("{}", style.apply_to(&self.value).dim())
        }
    }
}
