    use elysium_sdk::vtable_validate;

    #[repr(C)]
    struct VTable {
        drop: unsafe extern "C" fn(this: *const Networkable),
        release: unsafe extern "C" fn(this: *const Networkable),
        get_client_class: unsafe extern "C" fn(this: *const Networkable) -> *const u8,
        _pad1: vtable::Pad<3>,
        pre_data_update: unsafe extern "C" fn(this: *const Networkable, update_kind: UpdateKind),
        _pad2: vtable::Pad<2>,
        get_dormant: unsafe extern "C" fn(this: *const Networkable) -> bool,
        get_index: unsafe extern "C" fn(this: *const Networkable) -> i32,
        _pad3: vtable::Pad<2>,
        set_destroyed_on_recreate_entities: unsafe extern "C" fn(this: *const Networkable),
    }

    vtable_validate! {
        drop => 0,
        release => 1,
        get_client_class => 2,
        pre_data_update => 6,
        get_dormant => 9,
        get_index => 10,
        set_destroyed_on_recreate_entities => 13,
    }

    #[repr(C)]
    pub struct Networkable {
        vtable: &'static VTable,
    }
