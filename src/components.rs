#[derive(Default)]
pub struct Player {
    pub has_collided: Option<Collider>,
}

#[derive(Clone, Copy)]
pub enum Collider {
    Hazard,
    Princess,
}
