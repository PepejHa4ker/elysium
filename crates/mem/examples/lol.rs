unsafe fn foo() {
    let ptr = &5u8 as *const u8;

    println!("{}", ptr as isize);

    let ptr = elysium_mem::to_absolute(ptr, -1, 0);

    println!("{}", ptr as isize);
}

fn main() {
    unsafe {
        foo();
    }
}
