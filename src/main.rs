extern crate chrono;
extern crate libc;
#[macro_use]
extern crate nix;

use libc::c_int;

// ioctls, stolen from linux/rtc.h
ioctl!(read rtc_rd_time with 'p', 0x09; RtcTime);
ioctl!(write_ptr rtc_set_time with 'p', 0x0a; RtcTime);

/// Linux `struct rtc_time`
///
/// This structure is slightly shorter than other commonly used `struct tm*`.
/// It is assumed that the Rtc is kept at UTC.
///
/// Note that the resolution of the time struct is only seconds.
#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct RtcTime {
    /// Seconds
    pub tm_sec: c_int,
    /// Minutes
    pub tm_min: c_int,
    /// Hours
    pub tm_hour: c_int,
    /// Day of the month, first day is 0
    pub tm_mday: c_int,
    /// Month of the year, first month (January) is 0
    pub tm_mon: c_int,
    /// Year, starting at 118
    pub tm_year: c_int,
    /// unused
    pub tm_wday: c_int,
    /// unused
    pub tm_yday: c_int,
    /// unused
    pub tm_isdst: c_int,
}

impl From<RtcTime> for chrono::NaiveDateTime {
    fn from(rtc: RtcTime) -> chrono::NaiveDateTime {
        let d = chrono::NaiveDate::from_ymd(
            (rtc.tm_year + 1900) as i32,
            (rtc.tm_mon + 1) as u32,
            (rtc.tm_mday + 1) as u32,
        );
        let t =
            chrono::NaiveTime::from_hms(rtc.tm_hour as u32, rtc.tm_min as u32, rtc.tm_sec as u32);
        chrono::NaiveDateTime::new(d, t)
    }
}

#[test]
fn bindgen_test_layout_rtc_time() {
    assert_eq!(
        ::std::mem::size_of::<RtcTime>(),
        36usize,
        concat!("Size of: ", stringify!(RtcTime))
    );
    assert_eq!(
        ::std::mem::align_of::<RtcTime>(),
        4usize,
        concat!("Alignment of ", stringify!(RtcTime))
    );
}

fn main() {
    let rtc_dev = "/dev/rtc1";
}
