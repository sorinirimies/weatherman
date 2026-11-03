#![no_main]
//! Any timestamp the API could send, in any time zone name, formats as `HH:MM`
//! without panicking (even at the edges of chrono's date range).

use chrono::DateTime;
use libfuzzer_sys::fuzz_target;
use weatherman_core::format::format_local_time;

fuzz_target!(|data: &[u8]| {
    if data.len() < 8 {
        return;
    }
    let (secs, tz) = data.split_at(8);
    let secs = i64::from_le_bytes(secs.try_into().expect("8 bytes"));
    let Some(time) = DateTime::from_timestamp(secs, 0) else { return };
    let Ok(tz) = std::str::from_utf8(tz) else { return };
    let s = format_local_time(&time, tz);
    assert_eq!(s.len(), 5, "{s:?}");
    assert_eq!(&s[2..3], ":");
});
