// Change the function's stack pointer to point to new function Hello in a new stack.
// ret2libc wannabe
// basically run a function without calling it in main() by playing with pointers
use core::arch::asm;

const SSIZE: isize = 48;

#[derive(Debug, Default)]
// using C language's ABI because it is very stable than Rust's
#[repr(C)]
struct ThreadContext {
    rsp: u64,
}

fn hello() {
    println!("Im a function who is run without my consent.");
    loop {}
}

unsafe fn gt_switch(new: *const ThreadContext) {
    asm!(
        "mov rsp, [{0} + 0x00]",
        "ret",
        in(reg) new,
    )
}

fn main() {
    let mut ctx = ThreadContext::default();
    let mut stack = vec![0u8; SSIZE as usize];
    unsafe {
        let stack_bottom = stack.as_mut_ptr().offset(SSIZE);
        let sb_aligned = (stack_bottom as usize & !15) as *mut u8;
        std::ptr::write(sb_aligned.offset(-16) as *mut u64, hello as u64);
        ctx.rsp = sb_aligned.offset(-16) as u64;

        for i in 0..SSIZE {
            println!("mem: {}, val: {}",
            sb_aligned.offset(-i as isize) as usize,
            *sb_aligned.offset(-i as isize))
        }

        
        gt_switch(&mut ctx);
    }
}