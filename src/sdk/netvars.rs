use super::{Client, RecvProp};
use parking_lot::{RwLock, RwLockReadGuard, RwLockWriteGuard};
use std::collections::HashMap;
use std::lazy::SyncLazy;
use std::mem;
use std::ops::Deref;
use vmt::PointerExt;

pub type PropMap = HashMap<&'static str, RecvProp>;
pub type ClassMap = HashMap<&'static str, PropMap>;

pub static NETVARS: SyncLazy<RwLock<ClassMap>> = SyncLazy::new(|| RwLock::new(HashMap::new()));

pub fn read<'a>() -> RwLockReadGuard<'a, ClassMap> {
    NETVARS.read()
}

pub fn write<'a>() -> RwLockWriteGuard<'a, ClassMap> {
    NETVARS.write()
}

pub fn get_props(class: &str) -> Option<&'static PropMap> {
    unsafe { Some(crate::change_ref(read().get(class)?)) }
}

pub fn get(class: &str, prop: &str) -> Option<&'static RecvProp> {
    get_props(class)?.get(prop)
}

pub fn offset_of<'a>(class: &'a str, prop: &'a str) -> Option<isize> {
    get(class, prop).map(|prop| prop.offset as isize)
}

pub unsafe fn offset_unchecked<'a, T>(
    this: *const usize,
    class: &'a str,
    prop: &'a str,
) -> *const T {
    let offset = offset_of(class, prop).unwrap_unchecked();

    this.offset_bytes(offset) as *const T
}

pub unsafe fn offset<'a, T>(this: *const usize, class: &'a str, prop: &'a str) -> Option<*const T> {
    offset_of(class, prop).map(|offset| unsafe { this.offset_bytes(offset) as *const T })
}

pub fn set(client: &Client) {
    let mut classes = write();
    let client_classes = client.get_all_classes();

    for client_class in client_classes.iter() {
        let mut class = classes.entry(client_class.table_name()).or_default();

        for prop in client_class.props() {
            let name = prop.name();

            if name == "m_fFlags" {
                tracing::info!("found!!! {} -> {:?}", client_class.table_name(), prop);
            }

            class.insert(prop.name(), *prop);
        }
    }

    //tracing::info!("{:#?}", classes);
}
