use vptr::Pointer;

pub fn main() {
    unsafe {
        let offset = 5;
        let ptr = &offset as *const i32;

        println!("ptr = {:0x?}", ptr);
        println!("relative_offset = {:0x?}", ptr.relative_offset());
        println!("absolute = {:0x?}", ptr.to_absolute());
        println!("expected = {:0x?}", ptr.add_bytes(4 + 5));
    }
}
