#![no_std]
#![no_main]

use cortex_m_rt::entry;
use embassy_stm32::{pac};

use drone_firmware::comms::dshot::{self, Directionality::{Bidirectional, Normal}, Frame};

use panic_halt as _;

#[entry]
fn main() -> ! {
    let frame = Frame::throttle(500, false, Normal).unwrap();
    let mut buf = [0 as u16; 17];
    // frame.waveform(&mut buf, max_ccr);


    // Reset and Clock Controller (for enbaling peripherals and clock tree)
    let rcc = pac::RCC;
    // Flash interface block (for adjusting flash wait states). Flash is slower than the 170 MHz so we need to adjust
    let flash = pac::FLASH;
    // Power control block (for adjusting voltage scaling range). Modern MCUs have Dynamic Voltage and Frequency Scaling.
    // At lower voltages transistors switch slower, we need to set a high core voltage so as not to violate our own timing goals.
    let pwr = pac::PWR;
    // Block to control gpio port B
    let gpiob = pac::GPIOB;


    // Enable the external HSI16 (16 MHz). NOTE: rcc.cr is the clock control register
    rcc.cr().modify(|w| {
        w.set_hsion(true);
    });
    // Wait for the RCC to set the HSI ready bit true
    while !rcc.cr().read().hsirdy() {}


    // Configure for high voltage operation
    pwr.cr1().modify(|w|{
        w.set_vos(pac::pwr::vals::Vos::RANGE1);
    });

    
    // Set flash wait state for high-frequency (force the CPU to wait for more cycles to allow the slow flash fetching time)
    // 4 wait states is what the AI is suggesting, we'll look at the table before we run this
    flash.acr().modify(|w| {
        w.set_latency(pac::flash::vals::Latency::WS4);
    });

    

    // GPIO port B clock enabled
    rcc.ahb2enr().modify(|w| {
        w.set_gpioben(true);
    });
    // tim1 enabled 
    rcc.apb2enr().modify(|w| {
        w.set_tim1en(true);
    });
    //dma1 clock enabled
    rcc.ahb1enr().modify(|w| {
        w.set_dma1en(true);
    });

    //set B2 to alternate function mode
    gpiob.moder().modify(|w| {
        w.set_moder(2, pac::gpio::vals::Moder::ALTERNATE);
    });

    gpiob.afr(0).modify(|w|{
        w.set_afr(2, 2);
    });

    loop {
        cortex_m::asm::nop();
    }
}