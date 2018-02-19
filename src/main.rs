extern crate chrono;
extern crate libc;
#[macro_use]
extern crate nix;
use chrono::{Datelike, Timelike};

use libc::c_int;
use std::fs;
use std::os::unix::io::AsRawFd;

/// Basic epoch for dates.
///
/// All dates returned by the hardware clock are in offset of the epoch year,
/// which usually is 1900 (e.g. a value of `118` on `RtcTime::tm_year`
/// marks the year 2018).
pub const YEAR_EPOCH: i32 = 1900;

// ioctls, stolen from linux/rtc.h
ioctl!(read rtc_rd_time with 'p', 0x09; RtcTime);
ioctl!(write_ptr rtc_set_time with 'p', 0x0a; RtcTime);

/// Linux `struct rtc_time`
///
/// This structure is slightly shorter than other commonly used `struct tm*`.
/// It is assumed that the Rtc is kept at UTC.
///
/// Note that the resolution of the time struct is only seconds.
///
/// Conversion from and to `chrono::NaiveDateTime` is supported, Any resolution
/// beyond seconds will silently be discarded without rounding.
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
            rtc.tm_year as i32 + YEAR_EPOCH,
            (rtc.tm_mon + 1) as u32,
            (rtc.tm_mday + 1) as u32,
        );
        let t =
            chrono::NaiveTime::from_hms(rtc.tm_hour as u32, rtc.tm_min as u32, rtc.tm_sec as u32);
        chrono::NaiveDateTime::new(d, t)
    }
}

impl From<chrono::NaiveDateTime> for RtcTime {
    fn from(ct: chrono::NaiveDateTime) -> RtcTime {
        RtcTime {
            tm_sec: ct.time().second() as i32,
            tm_min: ct.time().minute() as i32,
            tm_hour: ct.time().hour() as i32,
            tm_mday: ct.date().day0() as i32,
            tm_mon: ct.date().month0() as i32,
            tm_year: ct.date().year() - YEAR_EPOCH,

            ..RtcTime::default()
        }
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
