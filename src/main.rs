use raytracer::ray::Ray;
use raytracer::utils::Vec3;

const WIDTH: usize = 200;
const HEIGHT: usize = 100;

fn main() {
    println!("P3\n{} {}\n255", WIDTH, HEIGHT);
    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);

    for j in (0..HEIGHT).rev() {
        for i in 0..WIDTH {
            let u = i as f32 / WIDTH as f32;
            let v = j as f32 / HEIGHT as f32;
            let ray = Ray::new(
                origin,
                lower_left_corner + (horizontal * u) + (vertical * v),
            );
            let color = ray.color();
            color.write_color();
        }
    }
}
