use num_traits::Num;
use rand::Rng;
use rand::distr::uniform::SampleUniform;

pub fn random<T>(min: T, max: T) -> T
where
    T: Num + PartialOrd + SampleUniform + Copy,
{
    let mut rng = rand::rng();

    rng.random_range(min..=max)
}
