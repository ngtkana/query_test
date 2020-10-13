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

pub mod queries {
    use query_test::{query, Query};
    use std::marker::PhantomData;

    #[query(fn(usize) -> T)]
    pub struct Get<T>(PhantomData<T>);

    #[query(fn(usize, T))]
    pub struct Set<T>(PhantomData<T>);
}

pub mod vector {
    #[derive(Debug, Clone, PartialEq)]
    pub struct Vector<T>(Vec<T>);

    mod vector_impl_gen {
        use super::{
            super::{
                helpers::Value,
                queries::{Get, Set},
            },
            Vector,
        };
        use query_test::{Gen, Help};
        use rand::Rng;

        impl<T, G: Help<Value<T>>> Gen<Get<T>, G> for Vector<T> {
            fn gen(&self, rng: &mut impl Rng) -> usize {
                rng.gen_range(0, self.0.len())
            }
        }

        impl<T, G: Help<Value<T>>> Gen<Set<T>, G> for Vector<T> {
            fn gen(&self, rng: &mut impl Rng) -> (usize, T) {
                (rng.gen_range(0, self.0.len()), G::help(rng))
            }
        }
    }

    mod vector_impl_solve {
        use super::{
            super::queries::{Get, Set},
            Vector,
        };
        use query_test::solve::{Mutate, Solve};

        impl<T: Clone> Solve<Get<T>> for Vector<T> {
            fn solve(&self, i: usize) -> T {
                self.0[i].clone()
            }
        }

        impl<T> Mutate<Set<T>> for Vector<T> {
            fn mutate(&mut self, (i, x): (usize, T)) {
                self.0[i] = x;
            }
        }
    }
}

pub mod gen_example {
    use super::helpers;
    use query_test::impl_help;
    use rand::Rng;

    pub struct G {}
    impl_help! {
        helpers::Len, |rng| rng.gen_range(1, 20);
        helpers::Value<u32>, |rng| rng.gen_range(0, 20);
    }
}
