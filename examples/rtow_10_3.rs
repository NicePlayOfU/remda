use remda::{
    camera::CameraBuilder,
    geometry::{GeometryList, Sphere},
    material::{Dielectric, Lambertian, Metal},
    prelude::*,
};

fn main() {
    env_logger::init();

    let mut world = GeometryList::default();

    world
        .add(Sphere::new(
            Point3::new(0.0, -100.5, -1.0),
            100.0,
            Lambertian::new(Color::newf(0.8, 0.8, 0.0)),
        ))
        .add(Sphere::new(
            Point3::new(0.0, 0.0, -1.0),
            0.5,
            Lambertian::new(Color::newf(0.1, 0.2, 0.5)),
        ))
        .add(Sphere::new(
            Point3::new(1.0, 0.0, -1.0),
            0.5,
            Metal::new(Color::newf(0.8, 0.6, 0.2)),
        ))
        .add(Sphere::new(
            Point3::new(-1.0, 0.0, -1.0),
            0.5,
            Dielectric::new(Color::newf(1.0, 1.0, 1.0), 1.5),
        ));

    let camera = CameraBuilder::default().aspect_ratio(2.0).build();

    camera
        .take_photo(world)
        .height(100)
        .samples(100)
        .shot(Some("rtow_10_3.ppm"))
        .unwrap();
}
