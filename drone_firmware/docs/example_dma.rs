#![no_std]
#![no_main]

// This example uses the raw Peripheral Access Crate (PAC) style on STM32G473.
// The goal is educational clarity:
//
// 1. Put the chip on a KNOWN clock tree.
// 2. Configure one GPIO pin as TIM1_CH1 output.
// 3. Configure TIM1 as PWM.
// 4. Configure DMA so each timer event writes the next duty value into CCR1.
// 5. Continuously repeat a test waveform buffer.
//
// This is the hardware pattern you later reuse for DShot.
//
// IMPORTANT:
// - The exact DMA request number for TIM1_CH1/TIM1_UP may differ depending on
//   which event you choose and which DMA channel you bind.
// - The exact board pad connected to the ESC may not actually be PA8.
// - You said you're OK figuring routing out yourself, so this example keeps
//   one concrete pin choice (PA8/TIM1_CH1) because it is valid on STM32G473.
//
// For STM32G473, the datasheet shows PA8 can be TIM1_CH1 using AF6.
// TIM1 is one of the advanced motor-control timers.
// The chip also has clock management, DMA, and DMAMUX blocks, so all of those
// matter here.

use cortex_m_rt::entry;
use stm32g4::stm32g473 as pac;

// A small repeating duty-cycle pattern.
// Each element is the value that will be written into TIM1_CCR1.
//
// ARR defines the total bit-cell length.
// CCR1 defines how long the output stays high before falling low.
//
// If ARR = 226 and:
// - CCR1 = 170  => output high for ~75% of the bit cell
// - CCR1 =  85  => output high for ~37% of the bit cell
//
// Those rough ratios are useful for DShot-like signalling later, but here we
// are just making a visible repeating pattern to prove the hardware path works.
static mut CCR_PATTERN: [u16; 16] = [
    170, 85, 170, 85,
    85, 170, 85, 170,
    170, 170, 85, 85,
    170, 85, 170, 85,
];

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();

    // ---------------------------------------------------------------------
    // RCC / CLOCK TREE SETUP
    // ---------------------------------------------------------------------
    //
    // Why do this at all?
    //
    // Because timer timing is always based on some clock source.
    // If you do not explicitly configure clocks, ARR/PSC/CCR are only counts,
    // not real time values.
    //
    // This example sets up:
    // - HSI16 internal oscillator as PLL source
    // - PLL as SYSCLK source
    // - SYSCLK = 170 MHz (a common STM32G4 top-speed setup)
    //
    // Then we keep AHB/APB prescalers at DIV1, which makes the timer clock
    // straightforward to reason about.
    //
    // NOTE:
    // The exact flash wait-state and regulator sequencing matter on real silicon.
    // This is a minimal learning example and may need small adjustments depending
    // on your startup environment / PAC field names / generated crate version.

    let rcc = dp.RCC;
    let flash = dp.FLASH;
    let pwr = dp.PWR;

    // 1) Enable the internal 16 MHz oscillator (HSI16).
    rcc.cr.modify(|_, w| w.hsion().set_bit());
    while rcc.cr.read().hsirdy().bit_is_clear() {}

    // 2) Configure voltage scaling for high clock operation.
    //
    // The exact field names can vary slightly by PAC version.
    // On G4, high-speed operation needs the regulator configured appropriately.
    pwr.cr1.modify(|_, w| w.vos().bits(0b01));

    // 3) Set flash wait states for high-frequency operation.
    //
    // Again, exact values can vary with voltage range and final frequency.
    // For learning purposes we are making the "high-speed flash config" step
    // explicit because it is part of real board bring-up.
    flash.acr.modify(|_, w| unsafe {
        w.latency().bits(4)
    });

    // 4) Turn PLL off before reconfiguring it.
    rcc.cr.modify(|_, w| w.pllon().clear_bit());
    while rcc.cr.read().pllrdy().bit_is_set() {}

    // 5) Configure PLL:
    //
    // HSI16 / M = 16 / 4 = 4 MHz
    // VCO = 4 MHz * N = 4 * 85 = 340 MHz
    // PLLR = 2 => SYSCLK = 170 MHz
    //
    // These numbers are chosen because they are easy to reason about.
    rcc.pllcfgr.modify(|_, w| unsafe {
        w.pllsrc().bits(0b10);   // HSI16 as PLL source
        w.pllm().bits(0b0011);   // divide by 4
        w.plln().bits(85);       // multiply by 85
        w.pllr().bits(0b00);     // divide by 2
        w.pllren().set_bit()     // enable PLLR output
    });

    // 6) Turn PLL back on and wait for lock.
    rcc.cr.modify(|_, w| w.pllon().set_bit());
    while rcc.cr.read().pllrdy().bit_is_clear() {}

    // 7) Keep bus prescalers simple: HCLK = SYSCLK, PCLK1 = HCLK, PCLK2 = HCLK.
    //
    // This makes TIM1's input clock easier to understand.
    rcc.cfgr.modify(|_, w| {
        w.hpre().div1();
        w.ppre1().div1();
        w.ppre2().div1()
    });

    // 8) Switch SYSCLK to PLL.
    rcc.cfgr.modify(|_, w| w.sw().pll());
    while !rcc.cfgr.read().sws().is_pll() {}

    // ---------------------------------------------------------------------
    // ENABLE PERIPHERAL CLOCKS
    // ---------------------------------------------------------------------
    //
    // Before touching a peripheral block, its bus clock must be enabled.
    // Otherwise register writes may do nothing or fault.
    rcc.ahb2enr.modify(|_, w| w.gpioaen().set_bit());
    rcc.apb2enr.modify(|_, w| w.tim1en().set_bit());
    rcc.ahb1enr.modify(|_, w| w.dma1en().set_bit());

    // ---------------------------------------------------------------------
    // GPIO SETUP: PA8 -> TIM1_CH1 (AF6)
    // ---------------------------------------------------------------------
    //
    // Datasheet alternate-function tables show PA8 can be TIM1_CH1 on AF6.
    // We configure:
    // - MODER = alternate function
    // - OTYPER = push-pull
    // - OSPEEDR = high speed
    // - PUPDR = no pull
    // - AFRH = AF6 for pin 8
    //
    // Push-pull is what you want for driving a digital control line to an ESC.
    // High speed here means the GPIO output driver can switch edges quickly.
    let gpioa = dp.GPIOA;

    gpioa.moder.modify(|r, w| {
        let mut bits = r.bits();
        bits &= !(0b11 << (8 * 2));
        bits |=  0b10 << (8 * 2);
        unsafe { w.bits(bits) }
    });

    gpioa.otyper.modify(|r, w| {
        let mut bits = r.bits();
        bits &= !(1 << 8); // 0 = push-pull
        unsafe { w.bits(bits) }
    });

    gpioa.ospeedr.modify(|r, w| {
        let mut bits = r.bits();
        bits &= !(0b11 << (8 * 2));
        bits |=  0b11 << (8 * 2); // very high speed
        unsafe { w.bits(bits) }
    });

    gpioa.pupdr.modify(|r, w| {
        let mut bits = r.bits();
        bits &= !(0b11 << (8 * 2)); // no pull-up/down
        unsafe { w.bits(bits) }
    });

    gpioa.afrh.modify(|r, w| {
        let mut bits = r.bits();
        bits &= !(0b1111 << ((8 - 8) * 4));
        bits |=  0b0110 << ((8 - 8) * 4); // AF6 for PA8
        unsafe { w.bits(bits) }
    });

    // ---------------------------------------------------------------------
    // TIM1 SETUP
    // ---------------------------------------------------------------------
    //
    // We want TIM1 to:
    // - run from a known clock
    // - generate PWM on CH1
    // - preload CCR1, so updates happen cleanly at period boundaries
    // - request DMA repeatedly
    //
    // We will use edge-aligned upcounting PWM mode 1.
    //
    // If TIM1 clock = 170 MHz and PSC = 0:
    // timer tick = 1 / 170 MHz = 5.882 ns
    //
    // If ARR = 226:
    // period = (ARR + 1) * tick = 227 * 5.882 ns ≈ 1.335 us
    //
    // That is close to a DShot600 bit time (~1.67 us), but not exact.
    // For now that is fine: this is a streaming PWM/DMA learning scaffold.
    //
    // Later, for exact DShot:
    // - choose target bit time
    // - compute ARR from timer clock
    // - choose CCR values for T0H and T1H
    let tim1 = dp.TIM1;

    // Disable timer before reconfiguring it.
    tim1.cr1.modify(|_, w| w.cen().clear_bit());

    // Prescaler = 0 means timer runs at full timer clock rate.
    tim1.psc.write(|w| unsafe { w.psc().bits(0) });

    // ARR = total period in timer ticks minus 1.
    tim1.arr.write(|w| unsafe { w.arr().bits(226) });

    // Start with CNT = 0 so we begin from a clean phase.
    tim1.cnt.write(|w| unsafe { w.cnt().bits(0) });

    // Enable ARR preload.
    //
    // This means ARR updates do not take effect halfway through a cycle.
    tim1.cr1.modify(|_, w| w.arpe().set_bit());

    // Configure Channel 1 as output compare / PWM mode 1.
    //
    // CC1S = 00  => channel configured as output
    // OC1M = 110 => PWM mode 1
    // OC1PE = 1  => preload enable for CCR1
    //
    // Preload is extremely important for streamed waveforms because it avoids
    // changing pulse width in the middle of the current bit cell.
    tim1.ccmr1_output().modify(|_, w| unsafe {
        w.cc1s().bits(0b00);
        w.oc1m().bits(0b110);
        w.oc1pe().set_bit()
    });

    // Set initial compare value before DMA starts.
    //
    // This means the first pulse has a sensible width even before the first
    // DMA transfer lands.
    tim1.ccr1.write(|w| unsafe { w.ccr1().bits(85) });

    // Enable capture/compare output for CH1.
    tim1.ccer.modify(|_, w| w.cc1e().set_bit());

    // Advanced timers gate outputs through the MOE bit in BDTR.
    // Without MOE, the timer can be counting internally but the pin may stay off.
    tim1.bdtr.modify(|_, w| w.moe().set_bit());

    // Generate an update event now.
    //
    // This forces registers with preload/shadow behaviour to latch in a known way.
    tim1.egr.write(|w| w.ug().set_bit());

    // ---------------------------------------------------------------------
    // DMA SETUP
    // ---------------------------------------------------------------------
    //
    // Goal:
    // move one u16 from CCR_PATTERN into TIM1_CCR1 on each timer DMA request.
    //
    // Important DMA concepts:
    // - CPAR = destination peripheral register address
    // - CMAR = source memory buffer address
    // - CNDTR = number of transfers
    // - MSIZE/PSIZE = transfer width
    // - MINC = advance through memory buffer
    // - CIRC = after the buffer ends, restart from the beginning
    //
    // Circular mode is what makes this a continuous repeating waveform.
    //
    // BIG WARNING:
    // The exact request source number in DMAMUX depends on the specific
    // timer event you use (TIM1_CH1, TIM1_UP, etc.) and the reference manual.
    // You will likely need to replace the placeholder request ID below.
    //
    // I am using DMA1 channel 1 here as an example.
    // On STM32G4, each DMA channel is paired with a DMAMUX channel.
    let dma1 = dp.DMA1;
    let dmamux = dp.DMAMUX1;

    let ccr = &dma1.ccr1;
    let cndtr = &dma1.cndtr1;
    let cpar = &dma1.cpar1;
    let cmar = &dma1.cmar1;

    // Disable the channel before touching its configuration.
    ccr.modify(|_, w| w.en().clear_bit());
    while ccr.read().en().bit_is_set() {}

    // Route a DMAMUX request into DMA1 Channel 1.
    //
    // PLACEHOLDER:
    // Replace the bits(...) value with the actual request ID from RM0440 for the
    // timer event you choose to drive transfers.
    //
    // Good real choices to investigate are:
    // - TIM1_CH1 DMA request
    // - TIM1_UP DMA request
    //
    // DShot implementations often care a lot about WHICH event triggers the write.
    dmamux.c0cr.modify(|_, w| unsafe {
        w.dmareq_id().bits(24)
    });

    // Destination address: TIM1_CCR1
    let tim1_ccr1_addr = &tim1.ccr1 as *const _ as u32;
    cpar.write(|w| unsafe { w.pa().bits(tim1_ccr1_addr) });

    // Source address: our duty pattern buffer
    let pattern_addr = unsafe { CCR_PATTERN.as_ptr() as u32 };
    cmar.write(|w| unsafe { w.ma().bits(pattern_addr) });

    // Number of 16-bit transfers in the buffer.
    cndtr.write(|w| unsafe { w.ndt().bits(unsafe { CCR_PATTERN.len() as u16 }) });

    // Configure the DMA channel.
    //
    // DIR   = 1 => memory-to-peripheral
    // MINC  = 1 => step through the memory buffer
    // PINC  = 0 => peripheral address stays fixed (always TIM1_CCR1)
    // MSIZE = 16-bit
    // PSIZE = 16-bit
    // CIRC  = 1 => repeat forever
    // PL    = high priority
    ccr.write(|w| unsafe {
        w.mem2mem().clear_bit();
        w.pl().bits(0b10);
        w.msize().bits(0b01);
        w.psize().bits(0b01);
        w.minc().set_bit();
        w.pinc().clear_bit();
        w.circ().set_bit();
        w.dir().set_bit()
    });

    // ---------------------------------------------------------------------
    // SELECT WHICH TIMER EVENT GENERATES DMA REQUESTS
    // ---------------------------------------------------------------------
    //
    // There are two common choices conceptually:
    //
    // 1. Update event (UDE):
    //    DMA request happens once per full timer period.
    //    This is often easier to reason about for streamed PWM data because the
    //    next CCR value is staged one period at a time.
    //
    // 2. Capture/compare event (CC1DE):
    //    DMA request happens on compare event timing for channel 1.
    //    This can work, but it is more subtle because the trigger moment is tied
    //    to the existing compare value.
    //
    // For a learning scaffold, UPDATE-event DMA is usually cleaner.
    tim1.dier.modify(|_, w| w.ude().set_bit());

    // Enable DMA channel after configuration is complete.
    ccr.modify(|_, w| w.en().set_bit());

    // Start TIM1.
    tim1.cr1.modify(|_, w| w.cen().set_bit());

    // ---------------------------------------------------------------------
    // MAIN LOOP
    // ---------------------------------------------------------------------
    //
    // At this point:
    // - TIM1 is running continuously
    // - TIM1 CH1 is driving PA8
    // - DMA is repeatedly rewriting CCR1 from CCR_PATTERN
    // - Because CIRC=1, the pattern repeats forever
    //
    // On a scope, you should see a constant-period waveform whose HIGH time
    // changes according to the values in CCR_PATTERN.
    //
    // That proves:
    // timer clocking, GPIO routing, advanced-timer output gating, DMA streaming,
    // and preload timing are all functioning together.
    loop {
        cortex_m::asm::nop();
    }
}