#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use esp_hal::{clock::ClockControl, embassy, peripherals::Peripherals, prelude::*};
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
        esp_hal::timer::TimerGroup::new(peripherals.TIMG0, &clocks)
    );

    spawner.spawn(run()).ok();

    loop {
        esp_println::println!("Bing!");
        Timer::after(Duration::from_millis(5_000)).await;
    }
}

// use embassy_net::tcp::TcpSocket;
// use embassy_net::{
//     Config, IpListenEndpoint, Ipv4Address, Ipv4Cidr, Stack, StackResources, StaticConfigV4,
// };
// use esp_hal as hal;
//
// use embassy_executor::Spawner;
// use embassy_time::{Duration, Timer};
// use esp_backtrace as _;
// use esp_println::{print, println};
// use esp_wifi::wifi::{AccessPointConfiguration, ClientConfiguration, Configuration};
// use esp_wifi::wifi::{
//     WifiApDevice, WifiController, WifiDevice, WifiEvent, WifiDeviceMode, WifiState,
// };
// use esp_wifi::{initialize, EspWifiInitFor};
//
// use hal::clock::ClockControl;
// use hal::Rng;
// use hal::{embassy, peripherals::Peripherals, prelude::*, timer::TimerGroup};
// use static_cell::make_static;
