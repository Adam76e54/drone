#![no_std]
#![no_main]

use drone_firmware::drivers::dshot;

use embassy_executor::Spawner;
use embassy_stm32::gpio::{Level, Output, Speed};
use embassy_time::Timer;
use panic_halt as _;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());

    // Start both OFF if the LEDs are active-low.
    let mut led0 = Output::new(p.PC15, Level::High, Speed::Low);
    let mut led1 = Output::new(p.PC14, Level::High, Speed::Low);

    loop {
        // Turn LED0 ON, LED1 OFF
        led0.set_low();
        led1.set_high();
        Timer::after_millis(500).await;

        // Turn LED0 OFF, LED1 ON
        led0.set_high();
        led1.set_low();
        Timer::after_millis(500).await;
    }
}