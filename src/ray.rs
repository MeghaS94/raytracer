use crate::utils::Vec3;

#[derive(Debug, PartialEq, Clone)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

pub fn ray_color(ray: &Ray) -> Vec3 {
    if hit_sphere(&Vec3::new(0.0, 0.0, -1.0), 0.5, &ray) {
        return Vec3::new(1.0, 0.0, 0.0);
    }
    let unit_direction = ray.direction.unit();
    let t = 0.5 * (unit_direction.y + 1.0);
    Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
}

pub fn hit_sphere(center: &Vec3, radius: f32, ray: &Ray) -> bool {
    let a = Vec3::dot(&ray.direction, &ray.direction);
    let b = Vec3::dot(&(ray.origin - *center), &ray.direction) * 2.0;
    let c = Vec3::dot(&(ray.origin - *center), &(ray.origin - *center)) - radius * radius;
    let d = b * b - 4.0 * a * c;
    return d >= 0.0;
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Self { origin, direction }
    }
    pub fn at(&self, t: f32) -> Vec3 {
        self.origin + self.direction * t
    }
}
