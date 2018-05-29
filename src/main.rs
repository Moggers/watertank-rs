#![feature(alloc)]
#![feature(global_allocator)]
#![feature(lang_items)]
#![no_std]
#![no_main]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#[macro_use]
extern crate alloc;
extern crate alloc_cortex_m;
extern crate cortex_m;
#[macro_use]
extern crate cortex_m_rt as rt;
extern crate cty;
extern crate panic_semihosting;
extern crate rtl8710_bindings;

mod wifi;

use alloc_cortex_m::CortexMHeap;
use rt::ExceptionFrame;
use cortex_m::asm;

#[global_allocator]
static ALLOCATOR: CortexMHeap = CortexMHeap::empty();

entry!(main);
fn main() -> ! {
    let wifi = wifi::Wifi {
        pass: "ssid",
        ssid: "pass",
        security: wifi::SecurityTypes::WPA2_AES_PSK,
    };
    wifi.connect();
    loop {}
}

exception!(HardFault, hard_fault);

fn hard_fault(ef: &ExceptionFrame) -> ! {
    panic!("HardFault at {:#?}", ef);
}

exception!(*, default_handler);

fn default_handler(irqn: i16) {
    panic!("Unhandled exception (IRQn = {})", irqn);
}

#[lang = "oom"]
#[no_mangle]
pub fn rust_oom() -> ! {
    asm::bkpt();

    loop {}
}
