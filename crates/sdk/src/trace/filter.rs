use super::TraceKind;

#[repr(C)]
struct VTable<F>
where
    F: super::Filter,
    F: 'static,
{
    should_hit_entity:
        unsafe extern "C" fn(this: *const Filter<F>, entity: *const (), mask: i32) -> bool,
    get_trace_kind: unsafe extern "C" fn(this: *const Filter<F>) -> TraceKind,
}

#[repr(C)]
pub struct Filter<F>
where
    F: super::Filter,
    F: 'static,
{
    vtable: &'static VTable<F>,
    filter: F,
}

impl<F> Filter<F>
where
    F: super::Filter,
{
    pub const fn new(filter: F) -> Self {
        Self {
            vtable: &VTable {
                should_hit_entity,
                get_trace_kind,
            },
            filter,
        }
    }
}

unsafe extern "C" fn should_hit_entity<F>(
    this: *const Filter<F>,
    entity: *const (),
    mask: i32,
) -> bool
where
    F: super::Filter,
{
    (*this).filter.should_hit_entity(entity, mask)
}

unsafe extern "C" fn get_trace_kind<F>(this: *const Filter<F>) -> TraceKind
where
    F: super::Filter,
{
    (*this).filter.get_trace_kind()
}
