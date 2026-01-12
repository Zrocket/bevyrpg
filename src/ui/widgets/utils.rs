use bevy::math::bounding::Aabb2d;

pub fn aabb_overlap(a: &Aabb2d, b: &Aabb2d) -> Aabb2d {
    Aabb2d {
        min: a.min.max(b.min),
        max: a.max.min(b.max),
    }
}
