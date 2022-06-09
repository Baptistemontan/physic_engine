use std::{time::Duration, mem};

use matrix::static_vector::StaticColumnVector;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct VerletObjectBase<const N:usize> {
    current_pos: StaticColumnVector<N>,
    old_pos: StaticColumnVector<N>,
    acceleration: StaticColumnVector<N>
}


impl<const N:usize> VerletObjectBase<N> {

    pub fn new(position: StaticColumnVector<N>) -> Self {
        Self {
            current_pos: position.clone(),
            old_pos: position,
            acceleration: StaticColumnVector::default()
        }
    }

    pub fn new_accelerated(position: StaticColumnVector<N>, acceleration: StaticColumnVector<N>) -> Self {
        Self {
            current_pos: position.clone(),
            old_pos: position,
            acceleration
        }
    }

    fn update(&mut self, dt: Duration) {
        let velocity = &self.current_pos - &self.old_pos;

        mem::swap(&mut self.current_pos, &mut self.old_pos);

        let dt_square = dt.as_secs_f64().powi(2);

        self.current_pos = velocity + &self.old_pos + &self.acceleration * dt_square;
    }

    fn accelerate(&mut self, acceleration: &StaticColumnVector<N>) {
        self.acceleration += acceleration;
    }
}

pub trait VerletObject<const N:usize> {
    fn get_verlet_infos_mut(&mut self) -> &mut VerletObjectBase<N>;

    fn get_verlet_infos(&self) -> &VerletObjectBase<N>;

    fn position(&self) -> &StaticColumnVector<N> {
        &self.get_verlet_infos().current_pos
    }

    fn position_mut(&mut self) -> &mut StaticColumnVector<N> {
        &mut self.get_verlet_infos_mut().current_pos
    }

    fn update(&mut self, dt: Duration) {
        self.get_verlet_infos_mut().update(dt);
    }

    fn accelerate(&mut self, acceleration: &StaticColumnVector<N>) {
        self.get_verlet_infos_mut().accelerate(acceleration);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod dummy_obj {
        use super::*;

        #[derive(Debug, Clone, PartialEq, Default)]
        pub struct DummyObj {
            verlet_infos: VerletObjectBase<3>
        }

        impl VerletObject<3> for DummyObj {
            fn get_verlet_infos_mut(&mut self) -> &mut VerletObjectBase<3> {
                &mut self.verlet_infos
            }

            fn get_verlet_infos(&self) -> &VerletObjectBase<3> {
                &self.verlet_infos
            }
        }
    }

    #[test]
    fn test_dyn_obj() {
        fn test(obj: &mut dyn VerletObject<3>) {
            obj.accelerate(&StaticColumnVector::from([2.0, 3.0, 4.0]));
            dbg!(obj.get_verlet_infos());
        }

        let mut a = dummy_obj::DummyObj::default();

        test(&mut a);
    }
}