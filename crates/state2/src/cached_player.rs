use providence_math::Vec3;
use providence_model::Bones;

pub struct CachedPlayer {
    pub bones: Bones,
    pub id: u64,
    pub index: i32,
    pub health: i32,
    pub magazine_ammo: i32,
    pub name: String,
    pub next_attack_available_after: f32,
    pub revolver_cock_time: f32,
    pub total_ammo: i32,
}

impl CachedPlayer {
    pub const fn new() -> CachedPlayer {
        let bones = Bones::zero();
        let id = 0;
        let index = 0;
        let health = 0;
        let magazine_ammo = 0;
        let name = String::new();
        let next_attack_available_after = 0.0;
        let revolver_cock_time = 0.0;
        let total_ammo = 0;

        Self {
            bones,
            health,
            id,
            index,
            magazine_ammo,
            name,
            next_attack_available_after,
            revolver_cock_time,
            total_ammo,
        }
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }
}
