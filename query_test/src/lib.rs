mod logger;
mod test_impl;

#[doc(hidden)]
pub use apply;
#[doc(hidden)]
pub use query_test_attr::*;

use rand::prelude::*;
use std::{cell::RefCell, fmt::Debug, marker::PhantomData};

#[macro_export]
macro_rules! impl_help {
    ($($ty:ty, $closure:expr;)*) => {
        $(
            impl $crate::Help<$ty> for G {
                fn help(rng: &mut impl Rng) -> <$ty as $crate::HelpMaterial>::Value {
                    use $crate::apply::Apply;
                    rng.apply($closure)
                }
            }
        )*
    };
}

pub trait Init<G> {
    fn init(rng: &mut impl Rng) -> Self;
}
pub trait FromBrute {
    type Brute;
    fn from_brute(brute: &Self::Brute) -> Self;
}
pub trait Query {
    type Param;
    type Output;
    const NAME: &'static str;
}
pub trait Gen<Q: Query, G> {
    fn gen(&self, rng: &mut impl Rng) -> Q::Param;
}
pub trait HelpMaterial {
    type Value;
}
pub trait Help<H: HelpMaterial> {
    fn help(rng: &mut impl Rng) -> H::Value;
}

pub mod solve {
    use crate::Query;
    pub trait Solve<Q: Query> {
        fn solve(&self, param: Q::Param) -> Q::Output;
    }
    pub trait SolveMut<Q: Query> {
        fn solve_mut(&mut self, param: Q::Param) -> Q::Output;
    }
    pub trait Mutate<Q: Query<Output = ()>> {
        fn mutate(&mut self, param: Q::Param);
    }
    pub trait Judge<Q: Query> {
        fn judge(&self, param: Q::Param, output: Q::Output) -> bool;
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Copy, Eq)]
pub enum Config {
    Short,
    Verbose,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Tester<R, B, F, G> {
    rng: RefCell<R>,
    brute: B,
    fast: F,
    marker: PhantomData<G>,
    config: Config,
}
