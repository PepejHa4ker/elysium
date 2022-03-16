use providence_util::virtual_table;

mod pri {
    #[repr(C)]
    pub struct VTable {
        foo: unsafe extern "C" fn(this: *const Entity),
        bar: unsafe extern "C" fn(this: *const Entity),
    }

    #[repr(C)]
    pub struct Entity {
        vtable: &'static VTable,
        health: i32,
    }

    unsafe extern "C" fn foo(this: *const Entity) {
        println!("foo");
        let health_ptr = (this as *const u8).add(8) as *mut i32;
        *health_ptr = 5;
        let health = *health_ptr;
        println!("health = {health:?}");
    }

    unsafe extern "C" fn bar(this: *const Entity) {
        println!("bar");
        let health_ptr = (this as *const u8).add(8) as *mut i32;
        let health = *health_ptr;
        println!("health = {health:?}");
    }

    pub unsafe fn get_entity() -> *const Entity {
        let vtable = Box::new(VTable { foo, bar });
        let vtable = &*Box::into_raw(vtable);
        let entity = Box::new(Entity { vtable, health: 0 });
        let entity = Box::into_raw(entity);

        entity
    }
}

#[repr(C)]
pub struct Entity(*const usize);

impl Entity {
    pub fn as_ptr(&self) -> *const usize {
        self.0
    }

    virtual_table! {
        fn foo[0]() -> ();
        fn bar[1]() -> ();
    }
}

fn main() {
    unsafe {
        let entity = Entity(pri::get_entity() as *const usize);

        entity.bar();
        entity.foo();
        entity.bar();
    }
}
