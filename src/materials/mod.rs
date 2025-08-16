pub mod lambertian;

//
//
// use rand::Rng;
// #[derive(Clone, Copy, Debug)]
// pub struct Metal {
//     pub albedo: Color,
//     pub fuzz: f64,
// }
//
// impl Metal {
//     pub fn new(albedo: Color, fuzz: f64) -> Self {
//         let fuzz = fuzz.clamp(0.0, 1.0);
//         Self { albedo, fuzz }
//     }
// }
//
// impl Material for Metal {
//     fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<MatBounce> {
//         let mut reflected = Vec3::reflect(r_in.direction, rec.normal);
//         reflected = Vec3::unit_vector(reflected) + (self.fuzz * Vec3::random_unit_vector());
//
//         let scattered = Ray::new(rec.p, reflected);
//         let attenuation = self.albedo;
//
//         if Vec3::dot(scattered.direction, rec.normal) > 0.0 {
//             Some(MatBounce {
//                 attenuation,
//                 scattered,
//             })
//         } else {
//             None
//         }
//     }
// }
//
// #[derive(Clone, Copy, Debug)]
// pub struct Dielectric {
//     pub refraction_index: f64,
// }
//
// impl Dielectric {
//     pub fn new(refraction_index: f64) -> Self {
//         Self { refraction_index }
//     }
//
//     fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
//         // Use Schlick's approximation for reflectance.
//         let mut r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
//         r0 *= r0;
//         r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
//     }
// }
//
// impl Material for Dielectric {
//     fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<MatBounce> {
//         let attenuation = Color::new(1.0, 1.0, 1.0);
//         let ri = if rec.front_face {
//             1.0 / self.refraction_index
//         } else {
//             self.refraction_index
//         };
//
//         let unit_direction = Vec3::unit_vector(r_in.direction);
//         let cos_theta = Vec3::dot(-unit_direction, rec.normal).min(1.0);
//         let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
//
//         let cannot_refract = ri * sin_theta > 1.0;
//         let scattered;
//         if cannot_refract || Dielectric::reflectance(cos_theta, ri) > rand::thread_rng().gen() {
//             scattered = Ray::new(rec.p, Vec3::reflect(unit_direction, rec.normal));
//         } else {
//             scattered = Ray::new(rec.p, Vec3::refract(unit_direction, rec.normal, ri));
//         }
//
//         Some(MatBounce {
//             attenuation,
//             scattered,
//         })
//     }
// }
