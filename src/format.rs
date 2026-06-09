//! Tiny helpers for turning raw numbers into human-friendly text.

use std::time::Duration;

const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];

/// Format a byte count like `1.4 GB` or `812 KB`.
pub fn bytes(n: u64) -> String {
    let mut size = n as f64;
    let mut unit = 0;

    while size >= 1024.0 && unit < UNITS.len() - 1 {
        size /= 1024.0;
        unit += 1;
    }

    if unit == 0 {
        format!("{n} {}", UNITS[0])
    } else {
        format!("{size:.1} {}", UNITS[unit])
    }
}

/// Format a duration like `820 ms` or `2.4 s`.
pub fn duration(d: Duration) -> String {
    let ms = d.as_millis();
    if ms < 1000 {
        format!("{ms} ms")
    } else {
        format!("{:.1} s", d.as_secs_f64())
    }
}

#[cfg(test)]
mod tests {
    use super::{bytes, duration};
    use std::time::Duration;

    #[test]
    fn small_values_stay_in_bytes() {
        assert_eq!(bytes(0), "0 B");
        assert_eq!(bytes(512), "512 B");
        assert_eq!(bytes(1023), "1023 B");
    }

    #[test]
    fn durations_switch_units() {
        assert_eq!(duration(Duration::from_millis(5)), "5 ms");
        assert_eq!(duration(Duration::from_millis(820)), "820 ms");
        assert_eq!(duration(Duration::from_millis(2400)), "2.4 s");
    }

    #[test]
    fn scales_up_to_each_unit() {
        assert_eq!(bytes(1024), "1.0 KB");
        assert_eq!(bytes(1536), "1.5 KB");
        assert_eq!(bytes(1024 * 1024), "1.0 MB");
        assert_eq!(bytes(3 * 1024 * 1024 * 1024), "3.0 GB");
    }
}
