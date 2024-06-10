#[test]

pub fn test_f1() {
    let mut mytuple: (u32, u32) = (55, 128);
    assert_eq!(&mut 55, p24::refs::f1(&mut mytuple, false));
    assert_eq!(&mut 128, p24::refs::f1(&mut mytuple, true));
}

#[test]
pub fn test_f2() {
    let mut s_f1: [u32; 6] = [23, 25, 19, 20, 30, 40];
    assert_eq!(&mut 40, p24::refs::f2(&mut s_f1, 5));
    assert_eq!(&mut 23, p24::refs::f2(&mut s_f1, 0));
    assert_eq!(&mut 20, p24::refs::f2(&mut s_f1, 3));
}

#[test]
pub fn test_f3() {
    let mut s_f1: [u32; 6] = [23, 25, 19, 20, 30, 40];
    assert_eq!(&mut 23, p24::refs::f3(&mut s_f1, 5));
    assert_eq!(&mut 40, p24::refs::f3(&mut s_f1, 0));
    assert_eq!(&mut 19, p24::refs::f3(&mut s_f1, 3));
}

#[test]
pub fn test_f4() {
    let mut s_f4: [u32; 13] = [23, 25, 19, 20, 30, 40, 15, 8, 18, 17, 11, 14, 50];

    let test_slice_f4: [&[u32]; 4] = [
        &[23, 25, 19],
        &[20, 30, 40],
        &[15, 8, 18],
        &[17, 11, 14, 50],
    ];
    assert_eq!(test_slice_f4, p24::refs::f4(&mut s_f4));
}
