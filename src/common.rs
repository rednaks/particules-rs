#[derive(Debug, Clone)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone)]
pub struct Size {
    pub width: f32,
    pub height: f32,
}

pub fn fit(val: f32, min: f32, max: f32) -> f32 {
    if val < min {
        return min;
    } else if val > max {
        return max;
    }
    val
}

#[derive(Debug, Clone)]
pub struct Stats {
    pub created_particules_count: i32,
    pub world_particules: i32,
    pub fps: i32,
}
