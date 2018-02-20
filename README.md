hwclock-rs
==========

The `hwclock` module provides a thin wrapper around kernel structs and ioctls to retrieve the current time from the hardware clock and convert it from and to a valid `chrono` data structure.

Example:

```rust
extern crate chrono;
extern crate hwclock;

fn main() {
   use hwclock::HwClockDev;

   let rtc = HwClockDev::open("/dev/rtc0").expect("could not open rtc clock");

   println!("{:?}", rtc);

   let time = rtc.get_time().expect("could not read rtc clock");
   println!("{:?}", time);

   println!("Setting clock ahead 30 seconds");
   let mut ct: chrono::NaiveDateTime = time.into();
   ct += chrono::Duration::seconds(30);

   // convert back to RtcTime and set it
   let ntime = ct.into();
   rtc.set_time(&ntime).expect("could not set rtc clock");

   println!("Rereading...");
   let time2 = rtc.get_time().expect("could not read rtc clock");

   println!("{:?}", time2);
}
```
