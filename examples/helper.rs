pub mod helpers {
    use query_test::{help_material, HelpMaterial};
    use std::marker::PhantomData;

    #[help_material(usize)]
    pub struct Len();

    #[help_material(T)]
    pub struct Value<T>(PhantomData<T>);

    #[help_material(T)]
    pub struct Key<T>(PhantomData<T>);
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
