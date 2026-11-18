#![no_std]
#![no_main]

use core::fmt::Write as _;

use cortex_m::delay::Delay;
use cortex_m_rt::entry;
use panic_halt as _;

use stm32f3xx_hal::{
    pac,
    prelude::*,
    serial::{config::Config, Serial},
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
    // Enable/configure GPIOA
    //
    // USART2 TX is connected to PA2.
    // USART2 RX is connected to PA3.
    // =====================================================
    let mut gpioa = dp.GPIOA.split(&mut rcc.ahb);

    // =====================================================
    // STEP 6:
    // Configure PA2 and PA3 for USART2 TX/RX
    // =====================================================
    let tx = gpioa.pa2.into_af_push_pull::<7>(
        &mut gpioa.moder,
        &mut gpioa.otyper,
        &mut gpioa.afrl,
    );
    let rx = gpioa.pa3.into_af_push_pull::<7>(
        &mut gpioa.moder,
        &mut gpioa.otyper,
        &mut gpioa.afrl,
    );

    // =====================================================
    // STEP 7:
    // Configure USART2
    // =====================================================
    let config = Config::default().baudrate(115_200.Bd());
    let mut serial = Serial::new(
        dp.USART2,
        (tx, rx),
        config,
        clocks,
        &mut rcc.apb1,
    );

    // =====================================================
    // STEP 8:
    // Configure SysTick based delay
    // =====================================================
    let mut delay = Delay::new(
        cp.SYST,
        clocks.sysclk().0,
    );

    // =====================================================
    // STEP 9:
    // Main loop
    //
    // Transmit "hello rust" every second
    // =====================================================
    loop {
        writeln!(serial, "hello rust").ok();
        delay.delay_ms(1_000_u32);
    }
}
