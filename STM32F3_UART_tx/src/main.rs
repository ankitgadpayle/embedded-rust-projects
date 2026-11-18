#![no_std]
#![no_main]

use cortex_m::delay::Delay;
use cortex_m_rt::entry;
use panic_halt as _;

use stm32f3xx_hal::{
    pac,
    prelude::*,
};

#[entry]
fn main() -> ! {
    // =====================================================
    // STEP 1:
    // Take ownership of STM32 peripheral registers
    // =====================================================
    let dp = pac::Peripherals::take().unwrap();

    // =====================================================
    // STEP 2:
    // Take ownership of Cortex-M core peripherals
    //
    // We need SYST (SysTick) from here.
    // =====================================================
    let cp = cortex_m::Peripherals::take().unwrap();

    // =====================================================
    // STEP 3:
    // Configure FLASH and RCC
    // =====================================================
    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();

    // =====================================================
    // STEP 4:
    // Freeze clock configuration
    //
    // The returned 'clocks' structure contains the actual
    // configured system clock frequencies.
    // =====================================================
    let clocks = rcc
        .cfgr
        .freeze(&mut flash.acr);

    // =====================================================
    // STEP 5:
    // Enable/configure GPIOE
    //
    // LED3 is connected to PE9.
    // =====================================================
    let mut gpioe = dp
        .GPIOE
        .split(&mut rcc.ahb);

    // =====================================================
    // STEP 6:
    // Configure all LED pins as push-pull outputs
    //
    // LED3  = PE9  (Red)
    // LED4  = PE8  (Blue)
    // LED5  = PE10 (Orange)
    // LED6  = PE15 (Green)
    // LED7  = PE11 (Green)
    // LED8  = PE14 (Orange)
    // LED9  = PE12 (Blue)
    // LED10 = PE13 (Red)
    // =====================================================
    let mut led3  = gpioe.pe9  .into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);

    // =====================================================
    // STEP 7:
    // Configure SysTick based delay
    // =====================================================
    let mut delay = Delay::new(
        cp.SYST,
        clocks.sysclk().0,
    );

    // =====================================================
    // STEP 8:
    // Initial LED state - all OFF
    // =====================================================
    led3.set_low().ok();
    // =====================================================
    // STEP 9:
    // Main superloop
    //
    // Sequence: LED3 → LED4 → LED6 → LED8 → LED10 → LED9 → LED7 → LED5 → repeat
    // =====================================================
    loop {
        // LED3 (PE9)
        led3.set_high().ok();
        delay.delay_ms(500_u32);
        led3.set_low().ok();
        delay.delay_ms(500_u32);

    }
}
