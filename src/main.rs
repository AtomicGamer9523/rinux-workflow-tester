#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rinuxcore::test_runner)]
#![reexport_test_harness_main = "test_main"]


extern crate rinuxcore;
extern crate config;
extern crate std3;

use rinuxcore::BootInfo;


#[rinuxcore::main]
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    rinuxcore::init(&boot_info);
    loop {}
}

#[panic_handler]
fn panic(info: &std3::panic::PanicInfo) -> ! {
    rinuxcore::print_err!("{}", info);
    rinuxcore::hlt_loop();
}
