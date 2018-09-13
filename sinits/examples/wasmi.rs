#![feature(lang_items)]
#![feature(asm)]
#![feature(const_fn)]
#![feature(unique)]
#![feature(alloc)]
#![no_std]

#[macro_use]
extern crate system;
extern crate spin;
extern crate selfalloc;
extern crate alloc;
extern crate wasmi;

use system::{CAddr};

#[lang="start"]
#[no_mangle]
#[allow(private_no_mangle_fns)]
fn start(_argc: isize, _argv: *const *const u8) {
    unsafe { system::set_task_buffer_addr(0x90001000); }
    unsafe { selfalloc::setup_allocator(CAddr::from(2), CAddr::from(3), 0x1000000000); }

    // Test allocator
    {
        use alloc::boxed::Box;
        use core::ops::Deref;
        let heap_test = Box::new(42);
        if heap_test.deref() != &42 {
            system::debug_test_fail();
        }
    }

    system_print!("hello, wasmi!");
    let module = wasmi::Module::from_buffer(&[0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00]).unwrap();

    let not_started = wasmi::ModuleInstance::new(
        &module,
        &wasmi::ImportsBuilder::default()
    ).unwrap();

    let instance = not_started.run_start(&mut wasmi::NopExternals).unwrap();
    system_print!("instance: {:?}", instance);

    system::debug_test_succeed();
}

#[no_mangle]
pub fn __truncdfsf2() {
    panic!("Floating points are not supported!");
}
