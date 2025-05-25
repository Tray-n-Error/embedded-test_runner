#![no_std]
#![no_main]

extern crate alloc;

use alloc::{string::{String, ToString}};
use embedded_alloc::LlffHeap as Heap;
use cortex_m_rt::entry;
use cortex_m_semihosting::{debug, hprintln};
// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// use panic_abort as _; // requires nightly
// use panic_itm as _; // logs messages over ITM; requires ITM support
// use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

#[global_allocator]
static HEAP: Heap = Heap::empty();

struct Test {
    name: String,
    func: fn() -> bool,
    result: bool,
    expected: bool,
}

impl Test {
    fn run(&mut self) {
        if (self.func)() {
            self.result = true
        } else {
            self.result = false
        }
    }
}
#[entry]
fn main() -> ! {
    {
        use core::mem::MaybeUninit;
        const HEAP_SIZE: usize = 1024;
        static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
        unsafe { HEAP.init(HEAP_MEM.as_ptr() as usize, HEAP_SIZE) }
    }
    
    fn should_be_ok() -> bool {
        true
    }
    fn failed_test() -> bool {
        false
    }
    fn everything_ok() -> bool {
        true
    }
    fn outside_temp_ok() -> bool {
        let outside_temp = 12;
        let required_temp = 9;
        outside_temp > required_temp
    }

    let test1 = Test { name: "should be OK".to_string(), func: should_be_ok, result: false, expected: true };
    let test2 = Test { name: "should fail".to_string(), func: failed_test, result: false, expected: false };
    let test3 = Test { name: "everything OK".to_string(), func: everything_ok, result: false, expected: true };
    let test4 = Test { name: "outside temperature OK".to_string(), func: outside_temp_ok, result: false, expected: true };

    let test_runner: [Test; 4] = [test1, test2, test3, test4];

    let mut test_counter = 0;
    let mut fail_counter = 0;
    let mut passed_counter = 0;
    let mut error_counter = 0;

    for mut test in test_runner {
        hprintln!("Running: {}", test.name);
        test.run();
            if test.result == test.expected {
                hprintln!("PASSED");
                passed_counter += 1;
            } else if test.result != test.expected {
                hprintln!("FAILED");
                fail_counter += 1;
            } else {
                hprintln!("ERROR OCCURRED");
                error_counter += 1;
            }
        test_counter += 1;
    }
    hprintln!("Passed: {}, Failed: {}, Error: {}, Tests run: {}", passed_counter, fail_counter, error_counter, test_counter);

    debug::exit(debug::EXIT_SUCCESS);

    loop {
        // code goes here...
    }
}