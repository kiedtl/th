// TODO: a more descriptive name?

use rand::prelude::*;
use serde::Deserialize;

#[derive(Copy, Clone, Debug, PartialEq, Deserialize)]
pub enum Value<T> {
    Manual(T),
    Random(T, T),
}

impl<T> Value<T>
where
    T: rand::distributions::uniform::SampleUniform + Copy
{
    pub fn get<R>(&self, r: &mut R) -> T
    where
        R: Rng
    {
        match self {
            Value::Manual(v) => *v,
            Value::Random(s, e) => r.gen_range(s, e),
        }
    }
}
