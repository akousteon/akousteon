use crate::components::{to_display, to_display_h_m_s, Speech};
use web_time::Duration;

#[test]
fn test_to_display() {
    assert_eq!("02:25", to_display(Duration::new(145, 0)));
    assert_eq!("00:45", to_display(Duration::new(45, 0)));
}

#[test]
fn test_to_display_h_m_s() {
    assert_eq!("03h42m22s", to_display_h_m_s(Duration::new(13342, 0)));
    assert_eq!("12m56s", to_display_h_m_s(Duration::new(776, 0)));
}

#[test]
fn test_export_to_csv() {
    let mut speech = Speech::new();
    speech.duration = Duration::new(63, 0);
    speech.category = "h".to_string();
    assert_eq!("63,\"h\"", speech.export_to_csv());
}
