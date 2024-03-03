#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use esp32c3_hal::{clock::ClockControl, embassy, peripherals::Peripherals, prelude::*};
use esp_backtrace as _;

#[embassy_executor::task]
async fn run() {
    loop {
        esp_println::println!("Hello world from embassy using esp-hal-async!");
        Timer::after(Duration::from_millis(1_000)).await;
    }
}

#[main]
async fn main(spawner: Spawner) {
    esp_println::println!("Init!");
    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    // #[cfg(feature = "embassy-time-systick")]
    // embassy::init(
    //     &clocks,
    //     esp32c3_hal::systimer::SystemTimer::new(peripherals.SYSTIMER),
    // );

    // #[cfg(feature = "embassy-time-timg0")]
    embassy::init(
        &clocks,
        esp32c3_hal::timer::TimerGroup::new(peripherals.TIMG0, &clocks).timer0,
    );

    spawner.spawn(run()).ok();

    loop {
        esp_println::println!("Bing!");
        Timer::after(Duration::from_millis(5_000)).await;
    }
}


// #![no_std]
// #![no_main]
// 
// extern crate alloc;
// use core::mem::MaybeUninit;
// use esp32c3_hal::{clock::ClockControl, peripherals::Peripherals, prelude::*, Delay};
// use esp_backtrace as _;
// use esp_println::println;
// 
// use esp_wifi::{initialize, EspWifiInitFor};
// 
// use esp32c3_hal::{systimer::SystemTimer, Rng};
// #[global_allocator]
// static ALLOCATOR: esp_alloc::EspHeap = esp_alloc::EspHeap::empty();
// 
// fn init_heap() {
//     const HEAP_SIZE: usize = 32 * 1024;
//     static mut HEAP: MaybeUninit<[u8; HEAP_SIZE]> = MaybeUninit::uninit();
// 
//     unsafe {
//         ALLOCATOR.init(HEAP.as_mut_ptr() as *mut u8, HEAP_SIZE);
//     }
// }
// #[entry]
// fn main() -> ! {
//     init_heap();
//     let peripherals = Peripherals::take();
//     let system = peripherals.SYSTEM.split();
// 
//     let clocks = ClockControl::max(system.clock_control).freeze();
//     let mut delay = Delay::new(&clocks);
// 
//     // setup logger
//     // To change the log_level change the env section in .cargo/config.toml
//     // or remove it and set ESP_LOGLEVEL manually before running cargo run
//     // this requires a clean rebuild because of https://github.com/rust-lang/cargo/issues/10358
//     // esp_println::logger::init_logger_from_env();
//     log::info!("Logger is setup");
//     println!("Hello world!");
//     let timer = SystemTimer::new(peripherals.SYSTIMER).alarm0;
//     let _init = initialize(
//         EspWifiInitFor::Wifi,
//         timer,
//         Rng::new(peripherals.RNG),
//         system.radio_clock_control,
//         &clocks,
//     )
//     .unwrap();
//     loop {
//         println!("Loop...");
//         delay.delay_ms(500u32);
//     }
// }
