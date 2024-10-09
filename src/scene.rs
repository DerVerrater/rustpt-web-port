
use web_sys::js_sys::Math::random;

use crate::primitives::{Ray, Vec2f, Vec3};

pub struct HitRecord{
    pub p: Vec3,
    pub normal: Vec3,
    pub material: Material,
    pub t: f32,
    pub front_face: bool,
}

impl HitRecord{
    pub fn set_face_normal(&mut self, r: Ray, outward_normal: Vec3) -> (){
        self.front_face = Vec3::dot(r.dir, outward_normal) < 0.0;
        self.normal = if self.front_face { outward_normal } else { -outward_normal };
    }
}

#[derive (Clone)]
pub enum Hittable {
    Sphere { center: Vec3, radius: f32, material: Material },
    HittableList { hittables: Vec<Hittable> }
}

impl Hittable {
    pub fn hit(&self, r: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        match self {
            Hittable::HittableList { hittables } => {
                hittables.iter()
                .map( |obj| -> Option<HitRecord> {
                    obj.hit(r, t_min, t_max)
                }).filter(|obj| obj.is_some())
                .min_by(|lhs, rhs| {
                    let lhs = lhs.as_ref().unwrap();
                    let rhs = rhs.as_ref().unwrap();
                    lhs.t.partial_cmp(&rhs.t).expect("Couldn't compare??")
                }).unwrap_or(None)
            }

            Hittable::Sphere { center, radius, material } => {
                let oc = r.orig - *center;
                let a = r.dir.length_squared();
                let half_b = Vec3::dot(oc, r.dir);
                let c = oc.length_squared() - radius * radius;
                let discriminant = half_b*half_b - a*c;

                if discriminant < 0.0 {
                    return None;
                }
                let sqrtd = discriminant.sqrt();

                // nearest root that lies within tolerance
                let mut root = (-half_b - sqrtd) / a;
                if root < t_min || root > t_max {
                    root = (-half_b + sqrtd) / a;
                    if root < t_min || root > t_max {
                        return None;
                    }
                }
                let mut record = HitRecord{
                    p: r.at(root),
                    normal: (r.at(root) - *center) / *radius,
                    material: *material,
                    t: root,
                    front_face: false,
                };
                let outward_normal = (record.p - *center) / *radius;
                record.set_face_normal(r, outward_normal);
                Some(record)
            }
        }
    }
    pub fn push(&mut self, item: Hittable) {
        if let Hittable::HittableList { hittables } = self {
            hittables.push(item);
        }
    }
}


#[derive(Copy, Clone, Debug)]
pub enum Material{
    Lambertian { albedo: Vec3 },
    Metal { albedo:Vec3, fuzz: f32 },
    Dielectric { index_refraction: f32 },
}

impl Material {
    pub fn scatter(
        &self,
        ray_in: Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        match self {
            Material::Lambertian { albedo } => {
                let scatter_dir = rec.normal + Vec3::rand_unit_vector();
                // The compiler might be smart enough to compute this ^^^ just once. In which case,
                // I don't need to do this weird dance. Oh well. It'll work.
                let scatter_dir = if scatter_dir.near_zero() {  // if near zero,
                    rec.normal                                  // replace with normal
                } else {
                    scatter_dir                                 // else preserve current
                };

                //TODO: Revisit this out-parameter pattern
                // It's a side effect of C++'s obtuse move semantics (and the RTIOW author not
                // using them at all)
                *scattered = Ray{
                    orig: rec.p,
                    dir: scatter_dir
                };
                *attenuation = *albedo; // deref on both sides? Wacky
                return true;
            },
            Material::Metal { albedo, fuzz } => {
                let reflected = Vec3::reflect(
                    Vec3::as_unit(ray_in.dir),
                    rec.normal
                );
                *scattered = Ray{
                    orig: rec.p,
                    dir: reflected + Vec3::rand_in_unit_sphere() * *fuzz,
                };
                *attenuation = *albedo;
                return Vec3::dot(scattered.dir, rec.normal) > 0.0;
            },
            Material::Dielectric { index_refraction } => {
                *attenuation = Vec3::ones();
                let refraction_ratio = if rec.front_face { 1.0 / index_refraction } else { *index_refraction };
                
                let unit_direction = Vec3::as_unit(ray_in.dir);
                let cos_theta = Vec3::dot(-unit_direction, rec.normal).min(1.0);
                let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

                let cannot_refract = refraction_ratio * sin_theta > 1.0;
                let distrib_zero_one = Vec2f::new(0.0, 1.0);
                let direction = if cannot_refract || Material::reflectance(cos_theta, refraction_ratio) > crate::lerp(distrib_zero_one, random() as f32) {
                    Vec3::reflect(unit_direction, rec.normal)
                } else {
                    Vec3::refract(unit_direction, rec.normal, refraction_ratio)
                };
                *scattered = Ray {
                    orig: rec.p,
                    dir: direction 
                };
                return true;
            },
        }
    }

    fn reflectance(cosine: f32, ref_idx: f32) -> f32 {
        // Schlick's approximation for reflectance.
        let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        let r0 = r0 * r0;
        return r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0);
    }
}

// Camera

pub fn degrees_to_radians(degrees: f32) -> f32 {
    degrees * std::f32::consts::PI / 180.0
}

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3, v: Vec3, /*w: Vec3,*/
    lens_radius: f32,
}

impl Camera {
    pub fn new(
        lookfrom: Vec3,
        lookat: Vec3,
        vup: Vec3,
        vfov: f32,
        aspect_ratio: f32,
        aperture: f32,
        focus_dist: f32
    ) -> Camera {
        let theta = degrees_to_radians(vfov);
        let h = (theta / 2.0).tan();
        let vp_height = 2.0 * h;
        let vp_width = aspect_ratio * vp_height;

        let w = Vec3::as_unit(lookfrom - lookat);
        let u = Vec3::as_unit(Vec3::cross(vup, w));
        let v = Vec3::cross(w, u);

        let orig = lookfrom;
        let horiz = u * vp_width * focus_dist;
        let verti = v * vp_height * focus_dist;
        let lower_left_corner = orig - horiz / 2.0 - verti / 2.0 - w * focus_dist;

        Camera{
            origin: orig,
            lower_left_corner,
            horizontal: horiz,
            vertical: verti,
            u, v, /* w,*/
            lens_radius: aperture / 2.0,
        }
    }

    pub fn get_ray(&self, s: f32, t: f32) -> Ray {
        let rd = Vec3::rand_in_unit_disk() * self.lens_radius;
        let offset = self.u * rd.x + self.v * rd.y;

        let dir = self.lower_left_corner
                + self.horizontal * s
                + self.vertical * t 
                - self.origin - offset;
        Ray{
            orig: self.origin + offset,
            dir,
        }
    }
}


pub struct Scene {
    pub camera: Camera,
    pub world: Hittable,
}

impl Scene {
    pub fn random_world() -> Hittable {
        let mat_ground = Material::Lambertian { albedo: Vec3::new(0.5, 0.5, 0.5) };
        let mut world = Hittable::HittableList { hittables : Vec::<Hittable>::new() };
        
        world.push( Hittable::Sphere { center: Vec3::new(0.0, -1000.0, 0.0), radius: 1000.0, material: mat_ground });
        
        let distrib_zero_one =  Vec2f::new(0.0, 1.0);
        for a in -11..11 {
            for b in -11..11 {
                let choose_mat = crate::lerp(distrib_zero_one, random() as f32);
                let center = Vec3 {
                    x: a as f32 + 0.9 * crate::lerp(distrib_zero_one, random() as f32),
                    y: 0.2,
                    z: b as f32 + 0.9 * crate::lerp(distrib_zero_one, random() as f32),
                };
                if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {

                    if choose_mat < 0.8 {
                        // diffuse
                        let albedo = Vec3::rand(distrib_zero_one) * Vec3::rand(distrib_zero_one);
                        let sphere_material = Material::Lambertian { albedo };
                        world.push(
                            Hittable::Sphere {
                                center,
                                radius: 0.2,
                                material: sphere_material,
                            }
                        );
                    } else if choose_mat < 0.95 {
                        // metal
                        let distr_albedo = Vec2f::new(0.5, 1.0);
                        let distr_fuzz = Vec2f::new(0.0, 0.5);

                        let albedo = Vec3::rand(distr_albedo);
                        let fuzz = crate::lerp(distr_fuzz, random() as f32);
                        let material = Material::Metal { albedo, fuzz };
                        world.push(
                            Hittable::Sphere {
                                center,
                                radius: 0.2,
                                material: material,
                            }
                        );
                    } else {
                        // glass
                        let material = Material::Dielectric { index_refraction: 1.5 };
                        world.push(
                            Hittable::Sphere{
                                center,
                                radius: 0.2,
                                material: material,
                            }
                        );

                    };
                }
            }
        }

        let material1 = Material::Dielectric { index_refraction: 1.5 };
        world.push( Hittable::Sphere{
            center: Vec3::new(0.0, 1.0, 0.0),
            radius: 1.0,
            material: material1
        });

        let material2 = Material::Lambertian { albedo: Vec3::new(0.4, 0.2, 0.1) };
        world.push( Hittable::Sphere {
            center: Vec3::new(-4.0, 1.0, 0.0),
            radius: 1.0,
            material: material2
        });

        let material3 = Material::Metal { albedo: Vec3::new(0.7, 0.6, 0.5), fuzz: 0.0 };
        world.push( Hittable::Sphere {
            center: Vec3::new(4.0, 1.0, 0.0),
            radius: 1.0,
            material: material3
        });
        world
    }
}