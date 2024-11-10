use crate::segment::Segment;

#[derive(Debug, Clone, PartialEq)]
pub enum Trend<T> {
    Rising { min: T, max: T },
    Constant(T),
    Falling { max: T, min: T },
}

pub fn find_trends<'a, 'b, T>(segments: &'a [Segment<'b, T>]) -> Vec<Trend<T>>
where
    T: Copy + Ord,
{
    let mut out = vec![];

    for segment in segments {
        if segment.len() < 2 {
            if segment.len() > 0 {
                out.push(Trend::Constant(segment[0]));
            }
            continue;
        }

        let first_val = segment[0];
        let last_val = segment[segment.len() - 1];

        match first_val.cmp(&last_val) {
            std::cmp::Ordering::Less => {
                // Rising trend
                if let Some(Trend::Rising { min: _, max }) = out.last_mut() {
                    // continue existing rising trend
                    *max = last_val;
                } else {
                    // start new rising trend
                    out.push(Trend::Rising {
                        min: first_val,
                        max: last_val,
                    });
                }
            }
            std::cmp::Ordering::Greater => {
                // Falling trend
                if let Some(Trend::Falling { max: _, min }) = out.last_mut() {
                    // continue existing falling trend
                    *min = last_val;
                } else {
                    // start new falling trend
                    out.push(Trend::Falling {
                        max: first_val,
                        min: last_val,
                    });
                }
            }
            std::cmp::Ordering::Equal => {
                // Constant trend
                if let Some(Trend::Constant(prev_val)) = out.last() {
                    if *prev_val != first_val {
                        out.push(Trend::Constant(first_val));
                    }
                } else {
                    out.push(Trend::Constant(first_val));
                }
            }
        }
    }

    if out.len() == 0 {
        return out;
    }

    // adjacent similar trends and handle transitions
    let mut i = 0;
    while i < out.len() - 1 {
        match (&out[i], &out[i + 1]) {
            (Trend::Rising { min, max: _ }, Trend::Rising { min: _, max }) => {
                out[i] = Trend::Rising {
                    min: *min,
                    max: *max,
                };
                out.remove(i + 1);
            }
            (Trend::Falling { max, min: _ }, Trend::Falling { max: _, min }) => {
                out[i] = Trend::Falling {
                    max: *max,
                    min: *min,
                };
                out.remove(i + 1);
            }
            (Trend::Constant(v1), Trend::Constant(v2)) if v1 == v2 => {
                out.remove(i + 1);
            }
            _ => {
                i += 1;
            }
        }
    }

    out
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::{
        segment::vector_dir_change::vector_dir_change, utils::noise::remove_small_segments,
    };

    #[test]
    pub fn test_find_trends() {
        let data = [
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            0, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5,
            5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 4, 4, 4, 4, 4, 3, 1,
            0, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6,
            6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 5, 3, 3, 3, 1, 1, 1, 1, 0,
            6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6,
            6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 5, 4, 2, 0,
        ];
        let segments = vector_dir_change(&data);
        let denoised = remove_small_segments(segments.as_slice(), 1);

        let pattern = find_trends(&denoised);

        use super::Trend::*;
        match pattern.as_slice() {
            &[
              Rising { min: 0, max: _ }, // -
              Constant(5), 
              Falling { max: _, min: 0 }, 
              Constant(6),
              Falling { max: _, min: 0 },
              Constant(6),
              Falling { max: _, min: 0 }
             ] =>  {}
            x => panic!("bad pattern {x:?}"),
        };
    }

    #[test]
    fn test_empty_segment() {
        let segments: Vec<Segment<i32>> = vec![];
        let trends = find_trends(&segments);
        assert!(trends.is_empty());
    }

    #[test]
    fn test_single_value_segment() {
        let data = [1];
        let segments = vector_dir_change(&data);
        let trends = find_trends(&segments);
        assert_eq!(trends, vec![Trend::Constant(1)]);
    }

    #[test]
    fn test_merge_similar_trends() {
        let data = [0, 1, 2, 3, 4, 5]; // continuous rising
        let segments = vector_dir_change(&data);
        let trends = find_trends(&segments);
        assert_eq!(trends, vec![Trend::Rising { min: 0, max: 5 }]);
    }
}
