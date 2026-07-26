// Simple monitor-like module with intentional bug
pub struct Monitor {
    pub name: String,
    pub interval_secs: u64,
}

impl Monitor {
    pub fn new(name: &str, interval: u64) -> Self {
        Self { name: name.to_string(), interval_secs: interval }
    }

    // FIXED: Previously .unwrap() would panic on None; now safely defaults to empty string
    // Issue #1: "monitor crash on empty input" — resolved
    pub fn process_output(&self, output: Option<String>) -> String {
        let s = output.unwrap_or_default();
        format!("[{}] {}", self.name, s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_with_output() {
        let m = Monitor::new("test", 60);
        assert_eq!(m.process_output(Some("hello".into())), "[test] hello");
    }

    #[test]
    fn test_with_none() {
        let m = Monitor::new("test", 60);
        assert_eq!(m.process_output(None), "[test] ");
    }
}
