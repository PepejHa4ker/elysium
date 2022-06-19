use elysium_math::Vec3;

const SURF_LIGHT: u32 = 0x0001; // value will hold the light strength
const SURF_SKY2D: u32 = 0x0002; // don't draw, indicates we should skylight draw 2d sky but not draw the 3D skybox
const SURF_SKY: u32 = 0x0004; // don't draw, but add to skybox
const SURF_WARP: u32 = 0x0008; // turbulent water warp
const SURF_TRANS: u32 = 0x0010;
const SURF_NOPORTAL: u32 = 0x0020; // the surface can not have a portal placed on it
const SURF_TRIGGER: u32 = 0x0040; // FIXME: This is an xbox hack to work around elimination of trigger surfaces, which breaks occluders
const SURF_NODRAW: u32 = 0x0080; // don't bother referencing the texture
const SURF_HINT: u32 = 0x0100; // make a primary bsp splitter
const SURF_SKIP: u32 = 0x0200; // completely ignore, allowing non-closed brushes
const SURF_NOLIGHT: u32 = 0x0400; // Don't calculate light
const SURF_BUMPLIGHT: u32 = 0x0800; // calculate three lightmaps for the surface for bumpmapping
const SURF_NOSHADOWS: u32 = 0x1000; // Don't receive shadows
const SURF_NODECALS: u32 = 0x2000; // Don't receive decals
const SURF_NOPAINT: u32 = SURF_NODECALS; // the surface can not have paint placed on it
const SURF_NOCHOP: u32 = 0x4000; // Don't subdivide patches on this surface
const SURF_HITBOX: u32 = 0x8000; // surface is part of a hitbox
const MASK_ALL: u32 = 0xFFFFFFFF;

fn trace_to_exit(
    start: Vec3,
    direction: Vec3,
    enter_summary: &Summary,
    exit_summary: &mut Summary,
    end: &mut Vec3,
) -> bool {
    println!("{start:?} {direction:?} {enter_summary:?} {exit_summary:?} {end:?}");

    let global = Global::handle();
    let mut distance = 0.0;

    while distance <= 90.0 {
        distance += 4.0;

        *end = start + direction * Vec3::splat(distance);

        let contents = global.ray_tracer().point_contents(
            *end,
            Contents::new().mask_shot_hull().hitbox(),
            ptr::null(),
        );

        if contents.has_mask_shot_hull() && contents.has_hitbox() {
            continue;
        }

        let new_end = *end - (direction * Vec3::splat(4.0));

        global.ray_tracer().trace_mut(
            &Ray::new(*end, new_end),
            Contents::new().mask_shot().hitbox(),
            None,
            exit_summary,
        );

        if exit_summary.start_solid && (exit_summary.surface.flags & SURF_HITBOX as u16) != 0 {
            unsafe {
                global.ray_tracer().trace_filtered_unchecked(
                    &Ray::new(*end, start),
                    Contents::new().mask_shot_hull().hitbox(),
                    exit_summary.entity_hit,
                    exit_summary,
                );
            }

            if (exit_summary.fraction <= 1.0 || exit_summary.all_solid) && !exit_summary.start_solid
            {
                *end = exit_summary.end;
                return true;
            }

            continue;
        }

        if !(exit_summary.fraction <= 1.0 || exit_summary.all_solid || exit_summary.start_solid)
            || exit_summary.start_solid
        {
            if exit_summary.entity_hit.is_null() {
                return true;
            }

            continue;
        }

        if (exit_summary.surface.flags & SURF_NODRAW as u16) != 0 {
            continue;
        }

        if exit_summary.plane.normal.dot(direction) <= 1.0 {
            let fraction = exit_summary.fraction * 4.0;

            *end = *end - (direction * Vec3::splat(fraction));

            return true;
        }
    }

    false
}

#[derive(Debug)]
pub struct ShotData {
    pub source: Vec3,
    pub enter_summary: Summary,
    pub direction: Vec3,
    pub filter: Option<Entity>,
    pub trace_length: f32,
    pub trace_length_remaining: f32,
    pub current_damage: f32,
    pub penetrate_count: i32,
}

impl ShotData {
    pub fn new() -> Self {
        Self {
            source: Vec3::zero(),
            enter_summary: Summary::new(),
            direction: Vec3::zero(),
            filter: None,
            trace_length: 0.0,
            trace_length_remaining: 0.0,
            current_damage: 0.0,
            penetrate_count: 0,
        }
    }

    pub fn handle_bullet_penetration(&mut self, weapon: &Weapon) -> bool {
        let global = Global::handle();
        let surface = match global
            .physics()
            .query(self.enter_summary.surface.properties as i32)
        {
            Some(surface) => surface,
            None => return true,
        };

        let enter_material = surface.properties.material;
        let enter_penetration_modifier = surface.properties.penetration_modifier;

        self.trace_length += self.trace_length_remaining * self.enter_summary.fraction;
        self.current_damage *= weapon.range_modifier().powf(self.trace_length * 0.002);

        if self.trace_length > 3000.0 || enter_penetration_modifier < 0.1 {
            self.penetrate_count = 0;
        }

        if self.penetrate_count <= 0 {
            return false;
        }

        let mut end = Vec3::zero();
        let mut exit_summary = Summary::new();

        if !trace_to_exit(
            /* start */ self.enter_summary.end,
            /* direction */ self.direction,
            /* enter_summary */ &self.enter_summary,
            /* exit_summary */ &mut exit_summary,
            /* end_pos */ &mut end,
        ) {
            return false;
        }

        let surface = match global.physics().query(exit_summary.surface.properties as _) {
            Some(surface) => surface,
            None => return true,
        };

        let exit_material = surface.properties.material;
        let exit_penetration_modifier = surface.properties.penetration_modifier;
        let mut final_damage_modifier: f32 = 0.16;
        let mut combined_penetration_modifier: f32 = 0.0;

        if self.enter_summary.contents.has_grate() || matches!(enter_material, 71 | 89) {
            final_damage_modifier = 0.05;
            combined_penetration_modifier = 3.0;
        } else {
            combined_penetration_modifier =
                (enter_penetration_modifier + exit_penetration_modifier) * 0.5;
        }

        if enter_material == exit_material {
            if matches!(exit_material, 85 | 87) {
                combined_penetration_modifier = 3.0;
            } else if exit_material == 76 {
                combined_penetration_modifier = 2.0;
            }
        }

        let v34 = f32::max(0.0, 1.0 / combined_penetration_modifier);
        let v35 = self.current_damage * final_damage_modifier
            + v34 * 3.0 * f32::max(0.0, (3.0 / weapon.penetration()) * 1.25);

        let mut thickness = (exit_summary.end - self.enter_summary.end).magnitude();

        thickness = (thickness * thickness * v34) / 24.0;

        let lost_damage = f32::max(0.0, v35 + thickness);

        if lost_damage > self.current_damage {
            return false;
        }

        if lost_damage >= 0.0 {
            self.current_damage -= lost_damage;
        }

        if self.current_damage < 1.0 {
            return false;
        }

        self.source = exit_summary.end;
        self.penetrate_count -= 1;

        // cant shoot through this
        true
    }

    pub fn simulate_shot(&mut self, local_player: &Player, weapon: &Weapon) -> bool {
        let global = Global::handle();

        let weapon_damage = weapon.damage();
        let weapon_range = weapon.range();
        let weapon_range_modifier = weapon.range_modifier();
        //let weapon_armor_ratio = weapon.armor_ratio();

        self.penetrate_count = 4;
        self.trace_length = 0.0;
        self.current_damage = weapon_damage;

        while self.penetrate_count > 0 && self.current_damage >= 1.0 {
            println!("current_damage = {}", self.current_damage);

            self.trace_length_remaining = weapon_range - self.trace_length;

            let end = self.source + self.direction * Vec3::splat(self.trace_length_remaining);
            let new_end = end + self.direction * Vec3::splat(4.0);

            global.ray_tracer().trace_mut(
                &Ray::new(self.source, end),
                Contents::new().mask_shot(),
                Some(&local_player.as_entity()),
                &mut self.enter_summary,
            );

            global.ray_tracer().trace_mut(
                &Ray::new(self.source, new_end),
                Contents::new().mask_shot(),
                self.filter.as_ref(),
                &mut self.enter_summary,
            );

            global.ray_tracer().trace_mut(
                &Ray::new(self.source, new_end),
                Contents::new().mask_shot(),
                Some(&local_player.as_entity()),
                &mut self.enter_summary,
            );

            if self.enter_summary.fraction == 1.0 {
                break;
            }

            if self.enter_summary.hit_group.is_hit() {
                //self.enter_summary.hit_group.damage_multipler(),
                //self.en
                //weapon.armor_ratio()
                //scaleDamage(shotData.enterTrace.hitgroup, player, info->weaponArmorRatio(), shotData.currentDamage);

                return true;
            }

            //if !self.handle_bullet_penetration(weapon) {
            break;
            //}
        }

        false
    }
}

fn calculate_angle(src: Vec3, dst: Vec3) -> Vec3 {
    let delta = src - dst;
    let hypot = (delta.x * delta.x + delta.y * delta.y).sqrt();

    let x = (delta.z / hypot).atan().to_degrees();
    let mut y = (delta.y / delta.x).atan().to_degrees();
    let z = 0.0;

    if delta.x >= 0.0 {
        y += 180.0;
    }

    Vec3::from_xyz(x, y, z)
}

fn angle_vector(angle: &Vec3, forward: &mut Vec3) {
    let x = angle.x.to_radians();
    let y = angle.y.to_radians();

    let (x_sin, x_cos) = x.sin_cos();
    let (y_sin, y_cos) = y.sin_cos();

    forward.x = x_cos * y_cos;
    forward.y = x_cos * y_sin;
    forward.z = -x_sin;
}

fn get_damage(local_player: &Player, weapon: &Weapon, destination: Vec3) -> f32 {
    let mut shot_data = ShotData::new();

    shot_data.source = local_player.eye_origin();
    shot_data.filter = Some(unsafe { Entity::new_unchecked(local_player.as_ptr() as *mut _) });

    let angle = calculate_angle(shot_data.source, destination);

    angle_vector(&angle, &mut shot_data.direction);

    shot_data.direction = shot_data.direction.normalize();

    if shot_data.simulate_shot(local_player, weapon) {
        shot_data.current_damage
    } else {
        -1.0
    }
}
