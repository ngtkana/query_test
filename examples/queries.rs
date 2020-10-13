use query_test::{query, Query};
use std::marker::PhantomData;

#[query(fn(usize) -> T)]
pub struct Get<T>(PhantomData<T>);
