#[derive(Debug, Clone)]
pub struct SupersetChart {
    pub title: String,
    pub chart_type: String,
}
impl SupersetChart {
    pub fn new(t: impl Into<String>, ct: impl Into<String>) -> Self {
        Self {
            title: t.into(),
            chart_type: ct.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_chart() {
        let c = SupersetChart::new("Users", "line");
        assert_eq!(c.chart_type, "line");
    }
}
