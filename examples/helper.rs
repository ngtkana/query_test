pub mod helpers {
    use query_test::HelpMaterial;
    use std::marker::PhantomData;

    pub struct Len();
    impl HelpMaterial for Len {
        type Value = usize;
    }
    pub struct Value<T>(PhantomData<T>);
    impl<T> HelpMaterial for Value<T> {
        type Value = T;
    }
    pub struct Key<T>(PhantomData<T>);
    impl<T> HelpMaterial for Key<T> {
        type Value = T;
    }
}

pub mod gen_example {
    use super::helpers;
    use query_test::Help;
    use rand::Rng;

    pub struct G {}
    impl Help<helpers::Len> for G {
        fn help(rng: &mut impl Rng) -> usize {
            rng.gen_range(1, 20)
        }
    }
    impl Help<helpers::Value<u32>> for G {
        fn help(rng: &mut impl Rng) -> u32 {
            rng.gen_range(0, 20)
        }
    }
}
