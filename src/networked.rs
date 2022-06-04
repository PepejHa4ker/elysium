use core::mem::MaybeUninit;
use elysium_sdk::client::{Client, Table};
use providence_networked::{Class, Entry};

#[derive(Debug)]
pub struct BaseAnimating {
    pub client_side_animation: usize,
}

#[derive(Debug)]
pub struct BaseEntity {
    pub render_mode: usize,
    pub team: usize,
}

#[derive(Debug)]
pub struct BasePlayer {
    pub aim_punch_angle: usize,
    pub health: usize,
    pub is_dead: usize,
    pub tick_base: usize,
    pub velocity: usize,
    pub view_offset: usize,
    pub view_punch_angle: usize,
}

#[derive(Debug)]
pub struct BaseWeapon {
    pub next_attack_available_after: usize,
    pub magazine: usize,
}

#[derive(Debug)]
pub struct Fog {
    pub color_primary: usize,
    pub density: usize,
    pub end: usize,
    pub far_z: usize,
    pub is_enabled: usize,
    pub start: usize,
}

#[derive(Debug)]
pub struct Item {
    pub index: usize,
}

#[derive(Debug)]
pub struct Player {
    pub armor: usize,
    pub eye_angle: usize,
    pub has_defuse_kit: usize,
    pub has_helmet: usize,
    pub is_immune: usize,
    pub is_scoped: usize,
    pub flags: usize,
    pub lower_body_yaw: usize,
    pub weapon: usize,
    pub money: usize,
    pub observer: usize,
}

#[derive(Debug)]
pub struct Weapon {
    pub revolver_cock_time: usize,
}

/// Networked variable manager.
#[derive(Debug)]
pub struct Networked {
    pub base_animating: BaseAnimating,
    pub base_entity: BaseEntity,
    pub base_player: BasePlayer,
    pub base_weapon: BaseWeapon,
    pub fog: Fog,
    pub item: Item,
    pub player: Player,
    pub weapon: Weapon,
}

impl Networked {
    pub fn from_client(client: &Client) -> Self {
        let mut this: Self = unsafe { MaybeUninit::zeroed().assume_init() };
        let top_level = client.get_all_classes();

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
}

/// Insert an entry we have interest in into our map.
fn insert_entry(this: &mut Networked, class: Class, entry: Entry, offset: usize) {
    match (class, entry) {
        // base_animating
        (Class::BaseAnimating, Entry::ClientSideAnimation) => {
            this.base_animating.client_side_animation = offset
        }

        // base_entity
        (Class::BaseEntity, Entry::RenderMode) => this.base_entity.render_mode = offset,
        (Class::BaseEntity, Entry::Team) => this.base_entity.team = offset,

        // base_player
        (Class::BasePlayer, Entry::AimPunchAngle) => this.base_player.aim_punch_angle = offset,
        (Class::BasePlayer, Entry::Health) => this.base_player.health = offset,
        (Class::BasePlayer, Entry::IsDead) => this.base_player.is_dead = offset,
        (Class::BasePlayer, Entry::TickBase) => this.base_player.tick_base = offset,
        (Class::BasePlayer, Entry::Velocity) => this.base_player.velocity = offset,
        (Class::BasePlayer, Entry::ViewOffset) => this.base_player.view_offset = offset,
        (Class::BasePlayer, Entry::ViewPunchAngle) => this.base_player.view_punch_angle = offset,

        // base_weapon
        (Class::BaseWeapon, Entry::NextAttackAvailableAfter) => {
            this.base_weapon.next_attack_available_after = offset
        }
        (Class::BaseWeapon, Entry::Magazine) => this.base_weapon.magazine = offset,

        // fog
        (Class::Fog, Entry::FogColorPrimary) => this.fog.color_primary = offset,
        (Class::Fog, Entry::FogDensity) => this.fog.density = offset,
        (Class::Fog, Entry::FogEnd) => this.fog.end = offset,
        (Class::Fog, Entry::FogFarZ) => this.fog.far_z = offset,
        (Class::Fog, Entry::FogIsEnabled) => this.fog.is_enabled = offset,
        (Class::Fog, Entry::FogStart) => this.fog.start = offset,

        // item
        (Class::Item, Entry::ItemIndex) => this.item.index = offset,

        // player
        (Class::Player, Entry::Armor) => this.player.armor = offset,
        (Class::Player, Entry::EyeAngle) => this.player.eye_angle = offset,
        (Class::Player, Entry::IsImmune) => this.player.is_immune = offset,
        (Class::Player, Entry::HasDefuseKit) => this.player.has_defuse_kit = offset,
        (Class::Player, Entry::HasHelmet) => this.player.has_helmet = offset,
        (Class::Player, Entry::IsScoped) => this.player.is_scoped = offset,
        (Class::Player, Entry::Flags) => this.player.flags = offset,
        (Class::Player, Entry::LowerBodyYaw) => this.player.lower_body_yaw = offset,
        (Class::Player, Entry::Weapon) => this.player.weapon = offset,
        (Class::Player, Entry::Observer) => this.player.observer = offset,
        (Class::Player, Entry::Money) => this.player.money = offset,

        // weapon
        (Class::Weapon, Entry::RevolverCockTime) => this.weapon.revolver_cock_time = offset,
        _ => {}
    }
}

/// Iterate the networked tables.
fn iterate_table(this: &mut Networked, table: &'static Table, class: Class, base_offset: usize) {
    // TODO: impl iterator for ISlice
    for property in table.properties().iter() {
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
