use crate::segment::Segment;

pub fn remove_small_segments<'a, 'b, T>(
    segments: &'a [Segment<'b, T>],
    threshold: usize,
) -> Vec<Segment<'b, T>> {
    segments
        .into_iter()
        .filter(|x| x.len() > threshold)
        .map(|x| *x)
        .collect::<Vec<_>>()
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::segment::vector_dir_change::vector_dir_change;

    #[test]
    pub fn test_remove_small_segments() {
        let segments = vector_dir_change(&[1, 2, 3, 4, 0, 5, 5]);
        assert_eq!(
            segments,
            vec![[1, 2, 3, 4].as_slice(), [0].as_slice(), [5, 5].as_slice()]
        );
        let denoised = remove_small_segments(segments.as_slice(), 1);
        assert_eq!(denoised, vec![[1, 2, 3, 4].as_slice(), [5, 5].as_slice()]);
    }
}
