# Embedded Rust Projects

Rust examples for the **STM32F303 Discovery kit** using `cortex-m`, `cortex-m-rt`, and `stm32f3xx-hal`.

## Examples

- `stm32f3_all_led_blink` — Blinks all on-board LEDs in sequence.
- `stm32f3_user_button_led4` — Toggles LED4 using the PA0 user button.
- `STM32F3_UART_tx` — Demonstrates UART transmission on the STM32F3.

## Requirements

- Rust and Cargo
- ARM Cortex-M cross-compilation target:

  ```bash
  rustup target add thumbv7em-none-eabihf
  ```

- An STM32F303 Discovery board and compatible flashing/debug tools

## Build

```bash
cd <project-directory>
cargo build
```
