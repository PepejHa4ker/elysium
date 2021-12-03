use vptr::Virtual;

#[repr(C)]
pub struct Foo {
    vtable: *const Vtable,
}

#[repr(C)]
pub struct Vtable {
    vtable: [*const (); 1],
}

fn print() {
    println!("amazing !");
}

pub fn main() {
    unsafe {
        let vfunc = print as *const ();
        let vtable = Box::new(Vtable { vtable: [vfunc] });
        let vtable = Box::into_raw(vtable);
        let foo = Foo { vtable };
        let ptr = &foo as *const Foo as *const ();
        let vfunc: fn() = ptr.vget(0);

        vfunc();
    }
}
