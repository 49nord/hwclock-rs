extern crate libc;

use libc::c_int;

/// Real-time clock time structure
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
#[repr(C)]
struct rtc_time {
    /// Seconds part of time
    tm_sec: c_int,
    /// Minutes part of time
    tm_min: c_int,
    /// Hour part of time
    tm_hour: c_int,
    /// Day of the month
    tm_mday: c_int,
    /// Month, starting at Jan=1
    tm_mon: c_int,
    /// Year
    tm_year: c_int,
    /// unused
    tm_wday: c_int,
    /// unused
    tm_yday: c_int,
    /// unused
    tm_isdst: c_int,
}

fn main() {
    let rtc_dev = "/dev/rtc1";
}
