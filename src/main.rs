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
#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub struct RtcTime {
    /// Seconds
    pub tm_sec: c_int,
    /// Minutes
    pub tm_min: c_int,
    /// Hours
    pub tm_hour: c_int,
    /// Day of the month (1-31)
    pub tm_mday: c_int,
    /// Months since January (0-11)
    pub tm_mon: c_int,
    /// Years since `YEAR_EPOCH` (1900)
    pub tm_year: c_int,
    /// unused, should be set to 0
    pub tm_wday: c_int,
    /// unused, should be set to 0
    pub tm_yday: c_int,
    /// unused, should be set to 0
    pub tm_isdst: c_int,
}

impl From<RtcTime> for chrono::NaiveDateTime {
    fn from(rtc: RtcTime) -> chrono::NaiveDateTime {
        let d = chrono::NaiveDate::from_ymd(
            rtc.tm_year as i32 + YEAR_EPOCH,
            (rtc.tm_mon + 1) as u32,
            rtc.tm_mday as u32,
        );
        let t = chrono::NaiveTime::from_hms(rtc.hour(), rtc.minute(), rtc.second());
        chrono::NaiveDateTime::new(d, t)
    }
}

impl Timelike for RtcTime {
    #[inline]
    fn second(&self) -> u32 {
        self.tm_sec as u32
    }

    #[inline]
    fn minute(&self) -> u32 {
        self.tm_min as u32
    }

    #[inline]
    fn hour(&self) -> u32 {
        self.tm_hour as u32
    }

    #[inline]
    fn nanosecond(&self) -> u32 {
        0
    }

    #[inline]
    fn with_hour(&self, hour: u32) -> Option<Self> {
        if hour < 24 {
            Some(RtcTime {
                tm_hour: hour as i32,
                ..*self
            })
        } else {
            None
        }
    }

    #[inline]
    fn with_minute(&self, minute: u32) -> Option<Self> {
        if minute < 60 {
            Some(RtcTime {
                tm_min: minute as i32,
                ..*self
            })
        } else {
            None
        }
    }

    #[inline]
    fn with_second(&self, second: u32) -> Option<Self> {
        if second < 60 {
            Some(RtcTime {
                tm_sec: second as i32,
                ..*self
            })
        } else {
            None
        }
    }

    #[inline]
    fn with_nanosecond(&self, _: u32) -> Option<Self> {
        Some(self.clone())
    }
}

impl From<chrono::NaiveDateTime> for RtcTime {
    fn from(ct: chrono::NaiveDateTime) -> RtcTime {
        RtcTime {
            tm_sec: ct.time().second() as i32,
            tm_min: ct.time().minute() as i32,
            tm_hour: ct.time().hour() as i32,
            tm_mday: ct.date().day() as i32,
            tm_mon: ct.date().month0() as i32,
            tm_year: ct.date().year() - YEAR_EPOCH,

            ..RtcTime::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn conversion_from_rtctime() {
        // Mon Feb 19 15:06:01 CET 2018
        // == Mon Feb 19 14:06:01 UTC 2018

        let rtc = RtcTime {
            tm_sec: 1,
            tm_min: 6,
            tm_hour: 14,
            tm_mday: 19,
            tm_mon: 1,
            tm_year: 118,
            tm_wday: 0,
            tm_yday: 0,
            tm_isdst: 0,
        };

        let ct = chrono::NaiveDateTime::new(
            chrono::NaiveDate::from_ymd(2018, 2, 19),
            chrono::NaiveTime::from_hms(14, 6, 1),
        );

        let rtc_from_ct: RtcTime = ct.into();
        let ct_from_rtc: chrono::NaiveDateTime = rtc.into();

        assert_eq!(rtc, rtc_from_ct);
        assert_eq!(ct, ct_from_rtc);
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

}

fn main() {
    let rtc_dev = "/dev/rtc1";
}
