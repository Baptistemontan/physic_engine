pub use verlet_derive;
pub use verlet_object;

#[cfg(test)]
mod tests {
    use crate::verlet_object::{VerletObject, VerletObjectBase};
    use verlet_derive::VerletObject;

    use super::*;
    #[derive(Debug, Default, VerletObject)]
    struct Foo {
        #[verlet_base]
        verlet_infos: crate::verlet_object::VerletObjectBase<2>,
        test: usize,
    }

    #[derive(Debug, Default, VerletObject)]
    struct FooGeneric<const N: usize> {
        #[verlet_base]
        verlet_infos: crate::verlet_object::VerletObjectBase<N>,
        test: usize,
    }

    #[test]
    fn test() {
        let bar = Foo {
            verlet_infos: VerletObjectBase::default(),
            test: 0,
        };
        assert_eq!(bar.verlet_infos, VerletObjectBase::default());
        dbg!(&bar);
        dbg!(bar.get_verlet_infos());
    }

    #[test]
    fn test_generic() {
        let mut bar: FooGeneric<4> = FooGeneric {
            verlet_infos: VerletObjectBase::default(),
            test: 0,
        };
        assert_eq!(bar.verlet_infos, VerletObjectBase::default());
        dbg!(&bar);
        dbg!(bar.get_verlet_infos_mut());
    }
}
