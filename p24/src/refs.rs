pub fn f1(s: &mut (u32, u32), flag: bool) -> &mut u32 {
    match flag {
        false => &mut s.0,
        true => &mut s.1,
    }
}

pub fn f2(s: &mut [u32], length: usize) -> &mut u32 {
    &mut s[length]
}

pub fn f3(s: &mut [u32], length: usize) -> &mut u32 {
    &mut s[s.len() - length - 1]
}

pub fn f4(s: &[u32]) -> [&[u32]; 4] {
    let mut result_slices: [&[u32]; 4] = [&[]; 4];
    if s.len() < 4 {
        result_slices = [&[], &[], &[], s]
    }
    result_slices[0] = &s[0..s.len() / 4];
    result_slices[1] = &s[(s.len() / 4)..s.len() / 2];
    result_slices[2] = &s[(s.len() / 2)..(s.len() * 3) / 4];
    result_slices[3] = &s[(s.len() * 3) / 4..];
    result_slices
}

#[cfg(test)]
mod tests {
    use super::{f1, f2, f3, f4};
    #[test]

    pub fn test_f1() {
        let mut mytuple: (u32, u32) = (55, 128);
        assert_eq!(&mut 55, f1(&mut mytuple, false));
        assert_eq!(&mut 128, f1(&mut mytuple, true));
    }

    #[test]
    pub fn test_f2() {
        let mut s_f1: [u32; 6] = [23, 25, 19, 20, 30, 40];
        assert_eq!(&mut 40, f2(&mut s_f1, 5));
        assert_eq!(&mut 23, f2(&mut s_f1, 0));
        assert_eq!(&mut 20, f2(&mut s_f1, 3));
    }

    #[test]
    pub fn test_f3() {
        let mut s_f1: [u32; 6] = [23, 25, 19, 20, 30, 40];
        assert_eq!(&mut 23, f3(&mut s_f1, 5));
        assert_eq!(&mut 40, f3(&mut s_f1, 0));
        assert_eq!(&mut 19, f3(&mut s_f1, 3));
    }

    #[test]
    pub fn test_f4() {
        let s_f4: [u32; 13] = [23, 25, 19, 20, 30, 40, 15, 8, 18, 17, 11, 14, 50];

        let test_slice_f4: [&[u32]; 4] = [
            &[23, 25, 19],
            &[20, 30, 40],
            &[15, 8, 18],
            &[17, 11, 14, 50],
        ];
        assert_eq!(test_slice_f4, f4(&s_f4));
    }
}
