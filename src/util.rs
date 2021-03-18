use std::cmp;

pub fn clamp<T>(value: T, max_value: T, min_value: T) -> T where T: Ord {
    return cmp::min(max_value, cmp::max(min_value, value));
}
