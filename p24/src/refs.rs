pub fn f1(s: &mut (u32, u32), flag: bool) -> &mut u32 {
    match flag {
        false => &mut s.0,
        true => &mut s.1,
    }
}

pub fn f2(s: &mut [u32], length: usize) -> &mut u32 {
    if length == 0 {
        return &mut s[0];
    }
    &mut s[length - 1]
}

pub fn f3(s: &mut [u32], length: usize) -> &mut u32 {
    if length == 0 {
        return &mut s[s.len() - 1];
    }
    &mut s[s.len() - length]
}

pub fn f4(s: &mut [u32]) -> &[[u32; 4]] {
    let number_of_slices = s.len() / 4;
    let blocks_of_slices: &mut [[u32; 4]] = &mut [];
    let mut intermediate_slice: [u32; 4] = [0, 0, 0, 0];
    for i in 0..number_of_slices {
        intermediate_slice.clone_from_slice(&s[4 * i..4 * (i + 1)]);
        blocks_of_slices[i].clone_from_slice(&intermediate_slice);
    }
    blocks_of_slices
}
