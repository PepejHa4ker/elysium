#![allow(incomplete_features)]
#![feature(const_convert)]
#![feature(const_maybe_uninit_uninit_array)]
#![feature(const_mut_refs)]
#![feature(const_ptr_offset_from)]
#![feature(const_refs_to_cell)]
#![feature(const_slice_from_raw_parts)]
#![feature(const_slice_index)]
#![feature(const_str_from_utf8_unchecked_mut)]
#![feature(const_trait_impl)]
#![feature(const_try)]
#![feature(generic_const_exprs)]
#![feature(maybe_uninit_uninit_array)]
#![feature(pointer_byte_offsets)]
#![feature(ptr_const_cast)]

pub use animation_layer::AnimationLayer;
pub use animation_state::AnimationState;
pub use beam::{Beam, BeamInfo, ViewRenderBeams};
pub use class::Class;
pub use client::Client;
pub use console::Console;
pub use engine::{Engine, PlayerInfo};
pub use entry::Entry;
pub use frame::Frame;
pub use id::SteamId;
pub use input::{Command, Input};
pub use input_system::InputSystem;
pub use interfaces::{InterfaceKind, Interfaces, LibraryKind};
pub use material::MaterialKind;
pub use network::{Flow, NetworkChannel};
pub use pad::Pad;
pub use panorama::{PanoramaEventRegistration, PanoramaUIEngine, UIEngine, UIPanel};
pub use render::{OverrideKind, Render};
pub use sound::{ActiveChannels, Channel};
pub use steam::SteamAPIContext;
pub use trace::{Filter, Trace, TraceKind};
pub use utl_map::UtlMap;
pub use utl_mem::UtlMem;
pub use utl_string::UtlString;
pub use utl_vec::UtlVec;
pub use var::{VarEntry, VarMap};
pub use weapon::{WeaponInfo, WeaponKind};

mod animation_layer;
mod animation_state;
mod beam;
mod class;
mod console;
mod engine;
mod entry;
mod frame;
mod input_system;
mod interfaces;
mod macros;
mod material;
mod pad;
mod panorama;
mod render;
mod sound;
mod steam;
mod utl_map;
mod utl_mem;
mod utl_string;
mod utl_vec;
mod var;
mod weapon;

pub mod client;
pub mod convar;
pub mod entity;
pub mod ffi;
pub mod id;
pub mod input;
pub mod network;
pub mod player_model;
pub mod trace;
