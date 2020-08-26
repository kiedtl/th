// misc math utilities

pub fn clamp<T>(val: T, min: T, max: T) -> T
where
    T: PartialOrd
{
    assert!(max >= min);
    if val < min {
        min
    } else if val > max {
        max
    } else { val }
}
