#![no_std]
#![feature(start)] 

// Required to use the `alloc` crate and its types, the `abort` intrinsic, and a
// custom panic handler.
#![feature(alloc, core_intrinsics, panic_implementation, lang_items, alloc_error_handler)]

extern crate alloc;
extern crate wee_alloc;

use alloc::boxed::Box;

// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// Need to provide a tiny `panic` implementation for `#![no_std]`.
// This translates into an `unreachable` instruction that will
// raise a `trap` the WebAssembly execution if we panic at runtime.
#[panic_handler]
#[no_mangle]
pub fn panic(_info: &::core::panic::PanicInfo) -> ! {
    unsafe {
        ::core::intrinsics::abort();
    }
}

// Need to provide an allocation error handler which just aborts
// the execution with trap.
#[alloc_error_handler]
#[no_mangle]
pub extern "C" fn oom(_: ::core::alloc::Layout) -> ! {
    unsafe {
        ::core::intrinsics::abort();
    }
}

#[start]
fn start(_argc: isize, _argv: *const *const u8) -> isize {
unsafe {
  let appname = _argv.offset(0);
  let mut chunk: Box<[u8]> = Box::new([0; 10]);

  let mut i:isize = 0;
  for i in 0..6 {
    chunk[i as usize] = **(appname.offset(i as isize)) ;
  }
  match &chunk[..6] {
      b"foobar" => 1,
       _ => 0
    }
}
}
