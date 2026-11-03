#![no_main]
//! The settings file is user-editable: any bytes either fail to parse or give
//! settings that survive a save/load round trip, and `add_location` keeps the
//! list trimmed, non-empty and free of case-insensitive duplicates.

use libfuzzer_sys::fuzz_target;
use weatherman_core::persistence::AppSettings;

fuzz_target!(|data: &[u8]| {
    let Ok(mut s) = serde_json::from_slice::<AppSettings>(data) else { return };
    let saved = serde_json::to_string(&s).expect("settings serialize");
    let back: AppSettings = serde_json::from_str(&saved).expect("saved settings load");
    assert_eq!(back.units, s.units);
    assert_eq!(back.locations, s.locations);

    let before = s.locations.len();
    let names: Vec<String> = s.locations.drain(..).collect();
    for n in &names {
        s.add_location(n);
    }
    assert!(s.locations.len() <= before);
    for (i, a) in s.locations.iter().enumerate() {
        assert!(!a.is_empty());
        assert_eq!(a, a.trim());
        assert!(!s.locations[..i].iter().any(|b| b.eq_ignore_ascii_case(a)));
    }
});
