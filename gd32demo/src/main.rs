#![no_std]
#![no_main]

use core::panic::PanicInfo;
use cortex_m::asm;
use cortex_m_rt::entry;
use gd32f3::gd32f303 as device;

#[panic_handler]
fn my_panic_handler(_panic: &PanicInfo) -> ! {
    loop {}
}

fn delay() {
    for _ in 0..10000 {
        asm::nop();
    }
}

#[entry]
fn main() -> ! {
    let p = device::Peripherals::take().unwrap();

    p.RCU.apb2en.modify(|_, w| w.pcen().set_bit());
    p.GPIOC.ctl0.modify(|_, w| unsafe { w.md3().bits(0x03) });

    loop {
        p.GPIOC.octl.modify(|_, w| w.octl13().set_bit());
        delay();
        p.GPIOC.octl.modify(|_, w| w.octl13().clear_bit());
        delay();
    }
}
