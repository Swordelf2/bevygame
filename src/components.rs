#[derive(Default)]
pub struct Player {
    pub has_collided: Option<Collider>,
}

#[derive(Clone, Copy, Debug)]
pub enum Collider {
    Hazard,
    Princess,
}

#[derive(Clone, Debug)]
pub struct Animation {
    pub len: u32,
}
