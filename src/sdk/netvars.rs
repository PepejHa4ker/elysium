use super::{Client, RecvProp, RecvTable};
use parking_lot::{RwLock, RwLockReadGuard, RwLockWriteGuard};
use std::collections::hash_map::{Entry, HashMap};
use std::lazy::SyncLazy;
use std::ops::Deref;
use std::{fmt, mem, ptr};
use vmt::PointerExt;

pub struct Offsets {
    map: HashMap<String, isize>,
}

impl Offsets {
    pub fn new() -> Self {
        let map = HashMap::new();

        Self { map }
    }

    pub fn insert(&mut self, prop: impl Into<String>, offset: isize) {
        self.map.insert(prop.into(), offset);
    }

    pub fn get(&self, prop: &str) -> Option<isize> {
        let offset = *self.map.get(prop)?;

        Some(offset)
    }
}

pub struct Tables {
    map: HashMap<String, Offsets>,
}

impl Tables {
    pub fn new() -> Self {
        let map = HashMap::new();

        Self { map }
    }

    pub fn insert(&mut self, table: impl Into<String>, prop: impl Into<String>, offset: isize) {
        match self.map.entry(table.into()) {
            Entry::Occupied(mut offsets) => offsets.get_mut().insert(prop, offset),
            Entry::Vacant(map) => {
                let mut offsets = Offsets::new();

                offsets.insert(prop, offset);
                map.insert(offsets);
            }
        }
    }

    pub fn get(&self, table: &str, prop: &str) -> Option<isize> {
        self.map.get(table)?.get(prop)
    }
}

pub struct Netvars {
    tables: SyncLazy<RwLock<Tables>, fn() -> RwLock<Tables>>,
}

impl Netvars {
    fn init() -> RwLock<Tables> {
        RwLock::new(Tables::new())
    }

    pub const fn new() -> Self {
        let tables = SyncLazy::new(Self::init as fn() -> RwLock<Tables>);

        Self { tables }
    }

    fn read<'a>(&'a self) -> RwLockReadGuard<'a, Tables> {
        self.tables.read()
    }

    fn write<'a>(&'a self) -> RwLockWriteGuard<'a, Tables> {
        self.tables.write()
    }

    pub fn insert(&self, table: impl Into<String>, prop: impl Into<String>, offset: isize) {
        self.write().insert(table, prop, offset);
    }

    pub fn get(&self, table: &str, prop: &str) -> Option<isize> {
        self.read().get(table, prop)
    }

    pub unsafe fn offset<T>(&self, ptr: *const usize, table: &str, prop: &str) -> *const T {
        if let Some(offset) = self.get(table, prop) {
            ptr.offset_bytes(offset) as *const T
        } else {
            ptr::null()
        }
    }
}

impl fmt::Debug for Netvars {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let mut set = fmt.debug_set();

        for (table, props) in self.read().map.iter() {
            for (prop, offset) in props.map.iter() {
                // TODO: make this less ugly.
                set.entry(&(
                    format!("{}.{}", table.as_str(), prop.as_str()),
                    format!("{:0x?}", offset),
                ));
            }
        }

        set.finish()
    }
}

pub static NETVARS: Netvars = Netvars::new();

pub fn iterate_table(props: &'static RecvTable, table: &'static str, offset: isize) {
    for prop in props.props() {
        if let Some(props) = prop.data_table() {
            iterate_table(props, table, offset + prop.offset as isize);
        }

        NETVARS.insert(table, prop.name(), offset + prop.offset as isize);
    }
}

pub fn set(client: &Client) {
    tracing::info!("Intialising netvars...");

    let all = client.get_all_classes();

    for class in all.iter() {
        if let Some(table) = class.recv_table {
            iterate_table(table, table.name(), 0);
        }
    }

    tracing::info!("{:#?}", NETVARS);
}

pub unsafe fn offset<T>(ptr: *const usize, table: &str, prop: &str) -> *const T {
    NETVARS.offset(ptr, table, prop)
}
