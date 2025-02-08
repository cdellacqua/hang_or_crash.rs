pub struct CustomSlice<T, const N: usize>(pub [T; N]);

#[cfg(test)]
mod tests {
    use super::*;

    #[inline(never)]
    fn bench_with_setup<I, O, S, R>(mut setup: S, mut routine: R)
    where
        S: FnMut() -> I,
        R: FnMut(I) -> O,
    {
        for _ in 0..10 {
            let input = setup();
            routine(input);
        }
    }

    #[test]
    fn test() {
        bench_with_setup(
            || CustomSlice([0; 1_000_000]),
            |my_slice| {
                assert_eq!(my_slice.0[10], 0);
            },
        );
    }
}
