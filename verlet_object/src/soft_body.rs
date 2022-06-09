use matrix::static_vector::StaticColumnVector;

use crate::verlet_object::VerletObject;

pub trait ConvexObject<const N:usize>: VerletObject<N> {
    fn points(&self) -> Vec<StaticColumnVector<N>>;

    fn collide_convex(&self, _other: &dyn ConvexObject<N>) -> Option<StaticColumnVector<N>> {
        unimplemented!()
    }
}

pub trait SphereObject<const N:usize>: VerletObject<N> {
    fn radius(&self) -> f64;

    fn collide_sphere(&self, other: &dyn SphereObject<N>) -> Option<StaticColumnVector<N>> {
        let collision_axis = self.position() - other.position();

        let distance = collision_axis.iter().map(|x| x.powi(2)).sum::<f64>().sqrt();
        let min_dist = self.radius() + other.radius();

        if distance < min_dist {
            let delta = min_dist - distance;
            let n = collision_axis * (delta / distance);
            Some(n)
        } else {
            None
        }

    }
}

pub trait ConcaveObject<const N:usize>: VerletObject<N> {
    fn parts(&self) -> Vec<Box<dyn ConvexObject<N>>>;
}

pub enum BodyType<'a, const N: usize> {
    Sphere(&'a dyn SphereObject<N>)
}

pub trait SoftBody<const N:usize>: VerletObject<N> {
    fn body_type(&self) -> BodyType<'_, N>;
    fn collide(&self, other: &dyn SoftBody<N>) -> Option<StaticColumnVector<N>>;

    fn uncollide(&mut self, other: &mut dyn SoftBody<N>) {
        if let Some(mut delta) = self.collide(other) {
            delta /= 2.0;
            *self.position_mut() += &delta;
            *other.position_mut() -= delta;
        }
    }
}

impl<const N: usize, T: SphereObject<N>> SoftBody<N> for T {

    fn body_type(&self) -> BodyType<'_, N> {
        BodyType::Sphere(self)
    }

    fn collide(&self, other: &dyn SoftBody<N>) -> Option<StaticColumnVector<N>> {
        match other.body_type() {
            BodyType::Sphere(other) => {
                self.collide_sphere(other)
            }
        }
    }
}