use crate::client::{Client, Table};

pub use api::{BaseAnimating, BaseEntity, BasePlayer, BaseWeapon, Item, Player, Weapon};

use class::Class;
use entry::Entry;

mod api;
mod class;
mod entry;

/// Networked variable manager.
#[derive(Debug)]
pub struct Networked {
    /// m_bClientSideAnimation
    base_animating: [usize; 1],

    // m_nRenderMode
    base_entity: [usize; 1],

    /// deadflag
    /// m_aimPunchAngle
    /// m_iHealth
    /// m_nTickBase
    /// m_vecVelocity\[\0\]
    /// m_vecViewOffset\[0\]
    /// m_viewPunchAngle
    base_player: [usize; 7],

    /// m_flNextPrimaryAttack
    /// m_iClip1
    base_weapon: [usize; 2],

    /// m_iItemDefinitionIndex
    item: [usize; 1],

    /// m_ArmorValue
    /// m_angEyeAngles\[0\]
    /// m_bGunGameImmunity
    /// m_bHasDefuser
    /// m_bHasHelmet
    /// m_bIsScoped
    /// m_fFlags
    /// m_flLowerBodyYawTarget
    /// m_hActiveWeapon
    /// m_hObserverTarget
    /// m_iAccount
    player: [usize; 11],

    /// m_flPostponeFireReadyTime
    weapon: [usize; 1],
}

impl Networked {
    pub fn from_client(client: &Client) -> Self {
        let mut this = Self {
            base_animating: [0; 1],
            base_entity: [0; 1],
            base_player: [0; 7],
            base_weapon: [0; 2],
            item: [0; 1],
            player: [0; 11],
            weapon: [0; 1],
        };

        let top_level = client.classes();

        // Iterate classes.
        for class in top_level.iter() {
            if let Some(table) = class.table {
                // Skip classes we are not interested in.
                if let Some(class) = Class::from_str(table.name()) {
                    iterate_table(&mut this, table, class, 0);
                }
            }
        }

        this
    }

    pub fn base_animating(&self) -> BaseAnimating<'_> {
        BaseAnimating(self)
    }

    pub fn base_entity(&self) -> BaseEntity<'_> {
        BaseEntity(self)
    }

    pub fn base_player(&self) -> BasePlayer<'_> {
        BasePlayer(self)
    }

    pub fn base_weapon(&self) -> BaseWeapon<'_> {
        BaseWeapon(self)
    }

    pub fn item(&self) -> Item<'_> {
        Item(self)
    }

    pub fn player(&self) -> Player<'_> {
        Player(self)
    }

    pub fn weapon(&self) -> Weapon<'_> {
        Weapon(self)
    }
}

/// Insert an entry we have interest in into our map.
// Bad rustfmt! Bad!
#[rustfmt::skip]
fn insert_entry(this: &mut Networked, class: Class, entry: Entry, offset: usize) {
    match (class, entry) {
        // base_animating
        (Class::BaseAnimating, Entry::ClientSideAnimation) => this.base_animating[0] = offset,

        // base_entity
        (Class::BaseEntity, Entry::RenderMode) => this.base_entity[0] = offset,

        // base_player
        (Class::BasePlayer, Entry::IsDead)         => this.base_player[0] = offset,
        (Class::BasePlayer, Entry::AimPunchAngle)  => this.base_player[1] = offset,
        (Class::BasePlayer, Entry::Health)         => this.base_player[2] = offset,
        (Class::BasePlayer, Entry::TickBase)       => this.base_player[3] = offset,
        (Class::BasePlayer, Entry::Velocity)       => this.base_player[4] = offset,
        (Class::BasePlayer, Entry::ViewOffset)     => this.base_player[5] = offset,
        (Class::BasePlayer, Entry::ViewPunchAngle) => this.base_player[6] = offset,

        // base_weapon
        (Class::BaseWeapon, Entry::NextAttackAvailableAfter) => this.base_weapon[0] = offset,
        (Class::BaseWeapon, Entry::Ammo)                     => this.base_weapon[1] = offset,

        // item
        (Class::Item, Entry::Index) => this.item[0] = offset,

        // player
        (Class::Player, Entry::Armor)              => this.player[0] = offset,
        (Class::Player, Entry::EyeAngle)           => this.player[1] = offset,
        (Class::Player, Entry::IsImmune)           => this.player[2] = offset,
        (Class::Player, Entry::HasDefuseKit)       => this.player[3] = offset,
        (Class::Player, Entry::HasHelmet)          => this.player[4] = offset,
        (Class::Player, Entry::IsScoped)           => this.player[5] = offset,
        (Class::Player, Entry::Flags)              => this.player[6] = offset,
        (Class::Player, Entry::LowerBodyYawTarget) => this.player[7] = offset,
        (Class::Player, Entry::Weapon)             => this.player[8] = offset,
        (Class::Player, Entry::ObserverTarget)     => this.player[9] = offset,
        (Class::Player, Entry::Money)              => this.player[10] = offset,

        // weapon
        (Class::Weapon, Entry::RevolverCockTime) => this.weapon[0] = offset,
        _ => {}
    }
}

/// Iterate the networked tables.
fn iterate_table(this: &mut Networked, table: &'static Table, class: Class, base_offset: usize) {
    // TODO: impl iterator for ISlice
    for property in table.properties.as_slice().iter() {
        if let Some(sub_table) = property.data_table() {
            // Recurse sub-tables.
            iterate_table(
                this,
                sub_table,
                class,
                base_offset + property.offset as usize,
            );
        }

        // Skip entries we are not interested in.
        if let Some(entry) = Entry::from_str(property.name()) {
            let offset = base_offset + property.offset as usize;

            insert_entry(this, class, entry, offset);
        }
    }
}
