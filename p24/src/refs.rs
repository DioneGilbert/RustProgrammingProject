pub fn f1(s: &mut (u32, u32), flag: bool) -> &mut u32 {
    match flag {
        false => &mut s.0,
        true => &mut s.1,
    }
}

pub fn f2(s: &mut [u32], length: usize) -> &mut u32 {
    &mut s[length - 1]
}

pub fn f3(s: &mut [u32], length: usize) -> &mut u32 {
    &mut s[s.len() - length]
}

pub fn f4(s: &mut [u32]) -> Vec<[u32; 4]> {
    let number_of_slices = s.len() / 4;
    let mut vector_of_slices: Vec<[u32; 4]> = vec![];
    let mut intermediate_slice: [u32; 4] = [0, 0, 0, 0];
    for i in 0..number_of_slices {
        intermediate_slice.clone_from_slice(&s[4 * i..4 * (i + 1)]);
        vector_of_slices.push(intermediate_slice);
    }
    vector_of_slices
}
