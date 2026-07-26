// Simple monitor-like module with intentional bug
pub struct Monitor {
    pub name: String,
    pub interval_secs: u64,
}

impl Monitor {
    pub fn new(name: &str, interval: u64) -> Self {
        Self { name: name.to_string(), interval_secs: interval }
    }

    // BUG: This unwrap will panic if output is empty
    // Issue: "monitor crash on empty input"
    pub fn process_output(&self, output: Option<String>) -> String {
        let s = output.unwrap();
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

    // Note: no test for empty case — that's where the bug lives
}
