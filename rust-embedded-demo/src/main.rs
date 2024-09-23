#![no_std]
#![no_main]
mod rtt_logger;
// pick a panicking behavior
// use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// use panic_abort as _; // requires nightly
// use panic_itm as _; // logs messages over ITM; requires ITM support
// use panic_semihosting as _; // logs messages to the host stderr; requires a debugger
use cortex_m_rt::entry;
use log::{debug, error, info, warn};
use panic_rtt_target as _;
use stm32h7xx_hal::{pac, prelude::*};

#[entry]
fn main() -> ! {
    rtt_logger::init(log::LevelFilter::Debug);
    // 通过获取 cortex_m 和 pac 外设的所有权来初始化 cp 和 dp 变量。
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    let pwr = dp.PWR.constrain();
    let pwrcfg = pwr.freeze();

    let rcc = dp.RCC.constrain();
    let ccdr = rcc.sys_ck(200.MHz()).freeze(pwrcfg, &dp.SYSCFG);

    let gpioe = dp.GPIOE.split(ccdr.peripheral.GPIOE);
    let mut led = gpioe.pe3.into_push_pull_output();

    let mut delay = cp.SYST.delay(ccdr.clocks);

    let mut cnt: u32 = 0;

    loop {
        // 点灯
        if cnt == 1 {
            info!("current cnt: {}", cnt);
        } else if cnt == 2 {
            warn!("current cnt: {}", cnt);
        } else if cnt == 3 {
            error!("current cnt: {}", cnt);
        } else if cnt > 100000 {
            panic!("panicked!")
        } else {
            debug!("current cnt: {}", cnt);
        }
        cnt += 1;

        led.toggle();

        delay.delay_ms(500_u16);
    }
}
