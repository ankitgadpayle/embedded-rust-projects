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
    // Enable/configure GPIOE (LED4) and GPIOA (user button)
    //
    // LED4  = PE8
    // USER  = PA0 (B1 user / wake-up button)
    // =====================================================
    let mut gpioe = dp
        .GPIOE
        .split(&mut rcc.ahb);
    let mut gpioa = dp
        .GPIOA
        .split(&mut rcc.ahb);

    // =====================================================
    // STEP 6:
    // Configure LED4 as a push-pull output
    // =====================================================
    let mut led4 = gpioe.pe8.into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);

    // =====================================================
    // STEP 7:
    // Configure the user button (PA0) as a floating input
    //
    // The button connects PA0 to GND when pressed, so it
    // reads LOW while pressed and HIGH when released.
    // =====================================================
    let user_button = gpioa.pa0.into_floating_input(&mut gpioa.moder, &mut gpioa.pupdr);

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
    // Initial LED state - OFF
    // =====================================================
    led4.set_low().ok();

    // =====================================================
    // STEP 10:
    // Main loop - toggle LED4 on each user button press
    //
    // The button is debounced and edge-detected so that a
    // single physical press toggles the LED exactly once.
    // =====================================================
    let mut led_on = false;
    let mut button_was_pressed = false;

    loop {
        // Debounce: wait for the line to settle, then read the
        // stable button state (true while pressed).
        delay.delay_ms(30_u32);
        let button_stable = user_button.is_low().unwrap();

        // Detect a fresh press (rising edge of "pressed").
        if button_stable && !button_was_pressed {
            // Toggle LED4.
            led_on = !led_on;
            if led_on {
                led4.set_high().ok();
            } else {
                led4.set_low().ok();
            }
        }

        // Remember the stable state for the next edge detection.
        button_was_pressed = button_stable;
    }
}
