#![cfg(not(all(target_feature = "soft-float", target_feature = "armv7a")))]

pub const LED_GREEN: u8 = 12;
pub const LED_ORANGE: u8 = 13;
pub const LED_RED: u8 = 14;
pub const LED_BLUE: u8 = 15;


pub const DELAY_COUNT_1MS: u16 = 1250;
pub const DELAY_COUNT_1SEC: u32 = 1000 * DELAY_COUNT_1MS as u32;
pub const DELAY_COUNT_125MS: u32 = 125 * DELAY_COUNT_1MS as u32;
pub const DELAY_COUNT_250MS: u32 = 250 * DELAY_COUNT_1MS as u32;
pub const DELAY_COUNT_500MS: u32 = 500 * DELAY_COUNT_1MS as u32;

pub fn delay(count: u32) {
    for _ in 0..count {}
}

pub fn led_init_all() {
    let p_rcc_ahb1enr = 0x40023830 as *mut u32;
    let p_gpiod_mode_reg = 0x40020C00 as *mut u32;

    unsafe {
        *p_rcc_ahb1enr |= 1 << 3;
        // configure LED_GREEN
        *p_gpiod_mode_reg |= 1 << (2 * LED_GREEN);
        *p_gpiod_mode_reg |= 1 << (2 * LED_ORANGE);
        *p_gpiod_mode_reg |= 1 << (2 * LED_RED);
        *p_gpiod_mode_reg |= 1 << (2 * LED_BLUE);
    }

    led_off(LED_GREEN);
    led_off(LED_ORANGE);
    led_off(LED_RED);
    led_off(LED_BLUE);
}

pub fn led_on(led_no: u8) {
    let p_gpiod_data_reg = 0x40020C14 as *mut u32;
    unsafe {
        *p_gpiod_data_reg |= 1 << led_no;
    }
}

pub fn led_off(led_no: u8) {
    let p_gpiod_data_reg = 0x40020C14 as *mut u32;
    unsafe {
        *p_gpiod_data_reg &= !(1 << led_no);
    }
}
