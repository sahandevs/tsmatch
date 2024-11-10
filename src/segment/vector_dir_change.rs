use super::Segment;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Dir {
    Pos,
    Neg,
}

fn get_dir<T>(a: &T, b: &T, last_dir: Dir) -> Dir
where
    T: Ord,
{
    match a.cmp(b) {
        std::cmp::Ordering::Less => Dir::Neg,
        std::cmp::Ordering::Equal => last_dir,
        std::cmp::Ordering::Greater => Dir::Pos,
    }
}

pub fn vector_dir_change<'a, T>(data: &'a [T]) -> Vec<Segment<'a, T>>
where
    T: Ord,
{
    let mut out = vec![];
    if data.len() <= 2 {
        return vec![data];
    }

    let mut dir = get_dir(&data[1], &data[0], Dir::Pos);
    let mut pos = 0;

    for i in 2..data.len() {
        let c_dir = get_dir(&data[i], &data[i - 1], dir);
        if c_dir == dir {
            if i == data.len() - 1 {
                out.push(&data[pos..]);
            }
            continue;
        }

        out.push(&data[pos..i]);
        dir = c_dir;
        pos = i;
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_segmentation_works() {
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

        let expected = vec![
            [
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                1, 1,
            ]
            .as_slice(),
            [0].as_slice(),
            [
                5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5,
                5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5,
            ]
            .as_slice(),
            [4, 4, 4, 4, 4, 3, 1, 0].as_slice(),
            [
                6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6,
                6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6,
            ]
            .as_slice(),
            [5, 3, 3, 3, 1, 1, 1, 1, 0].as_slice(),
            [
                6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6,
                6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6,
            ]
            .as_slice(),
        ];

        for (a, b) in vector_dir_change(data.as_slice()).iter().zip(expected) {
            assert_eq!(*a, b);
        }
    }
}
