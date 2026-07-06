#![no_std]
#![no_main]

use core::clone;

use cortex_m_rt::entry;
use embassy_stm32::{dac::Value, pac::{self, common::W, rcc::vals}, rcc::PllSource::HSI};

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
    // TIM20
    let tim20 = pac::TIM20;


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
    // Set boost mode 
    pwr.cr5().modify(|w| {
        w.set_r1mode(false);
    });
    // Wait for the voltage scaling is finished 
    while pwr.sr2().read().vosf() {}
    
    // Set flash wait state for high-frequency (force the CPU to wait for more cycles to allow the slow flash fetching time)
    // 4 wait states is what the AI is suggesting, we'll look at the table before we run this
    flash.acr().modify(|w| {
        w.set_latency(pac::flash::vals::Latency::WS4);
    });


    // turn off the pll (phase-locked loop) before we set it (this is safer for the hardware)
    rcc.cr().modify(|w|{
        w.set_pllon(false);
    });
    // Wait for the hardware flag to come off
    while rcc.cr().read().pllrdy() {}


    // configure pll:
    rcc.pllcfgr().modify(|w| {
        w.set_pllsrc(HSI); // Set the HSI as source 16 MHz
        w.set_pllm(vals::Pllm::DIV4); // Divide input by 4 (16/4 = 4 MHz) so the Voltage-Controlled Oscillator can handle it in
        w.set_plln(vals::Plln::MUL85); // Multiply by 85 (4 * 85 = 340 Mhz) so the Voltage-Controlled Oscillator can handle it out
        w.set_pllr(vals::Pllr::DIV2); // Output 170 MHz (chips max)
        w.set_pllren(true); // Enable PLL R Output to be used as sysclk

    });

    // turn pll back on
    rcc.cr().modify(|w| {
        w.set_pllon(true);
    });
    // Wait for hardware flag to come on
    while rcc.cr().read().pllrdy() {}


    // Set prescalers to 1 for simplicity 
    rcc.cfgr().modify(|w|{
        w.set_hpre(vals::Hpre::DIV1); // ahb bus prescalar
        w.set_ppre1(vals::Ppre::DIV1); // apb1 bus prescalar
        w.set_ppre2(vals::Ppre::DIV1); // apb2 bus prescalar
    });

    // Select PLL as the sysclk
    rcc.cfgr().modify(|w| {
        w.set_sw(vals::Sw::PLL1_R);
    });
    // 
    while rcc.cfgr().read().sws() != vals::Sw::PLL1_R {}

    
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


    // NOTE we're going to use TIM20 CH1

    // Turn off timer before reconfiguring
    tim20.cr1().modify(|w| {
        w.set_cen(false);
    });
    tim20.psc().modify(|w| {
        *w = 0u16; // for some reason there's no writer proxy for the prescalar (note that stm32 divides by psc + 1 so psc=1 means division by 1)
    });

    // NOTE: at this point 1/(170 MHz) = 5.882 ns so the period will be = (ARR + 1) * 5.882 ns
    // We want period = 3.33 us for DShot300 ==> 5.882 / (ARR + 1) = 333 ns  solves to ARR = 565
    // NOTE: that means a 0 lasts about 212 ticks and 1 lasts 425 ticks for our DShot300
    const ARR: u16 = 565;
    tim20.arr().modify(|w| {
        w.set_arr(ARR);
    });

    // Start timer count at 0
    tim20.cnt().modify(|w| {
        w.set_cnt(0);
    });

    // Enable the arr preload register so ARR updates only happen on the clock edges, not mid-cycle
    tim20.cr1().modify(|w| {
        w.set_arpe(true);
    });

    // Now we set up the timer channel. 
    // Think of a channel as a programmable comparator hooked to an output pin
    // Each CHx on a TIMy will have a few key registers:
    // 1. CCRx: the value that the timer actually compares ARR against
    // 2. CCRMx: the Capture/Compare Mode Register  which sets exactly what output settings
    // 3. CCERx: the output enable bits for turning things on off.  
    // 
    // For some reason the API starts asking us to use usize's to select which part of each thing we want
    // eg. there's 4 channels on each timer, CCMR1 for CH1-2 and CCMR2 for CH3-4
    // then ccmr1 -> css will have two halves: one for CH1, the other for CH2
    tim20.ccmr_output(0).modify(|w| {
        // Capture Compare Selection: CH1 set to OUTPUT 
        w.set_ccs(0, pac::timer::vals::CcmrOutputCcs::OUTPUT);
        // Output compare mode. PWM mode 1 means output is active when count < CCR, mode 2 is opposite
        w.set_ocm(0, pac::timer::vals::Ocm::PWM_MODE1);
        // Enable preload
        w.set_ocpe(0, true);
    });

    // Start ccr1 off at 33% duty (which is a 0 in DSHOT)
    tim20.ccr(0).modify(|w| {
        w.set_ccr(ARR / 3);
    });

    // enable capture/compare on channel 1
    tim20.ccer().modify(|w| {
        w.set_cce(0, true);
    });

    

    loop {
        cortex_m::asm::nop();
    }
}