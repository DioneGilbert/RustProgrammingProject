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

pub fn f4(s: &mut [u32]) -> [&[u32]; 4] {
    let slice_length: usize = s.len() / 4;
    let mut result_slices: [&[u32]; 4] = [&[]; 4];
    if s.len() < 4 {
        result_slices = [&[], &[], &[], s]
    }
    for i in 0..3 {
        result_slices[i] = &s[slice_length * i..slice_length * (i + 1)];
    }
    result_slices[3] = &s[3 * slice_length..];
    result_slices
}
