// use p24::refs;

#[test]

pub fn test_f1() {
    let mut mytuple: (u32, u32) = (55, 128);
    assert_eq!(&mut 55, p24::refs::f1(&mut mytuple, false));
    assert_eq!(&mut 128, p24::refs::f1(&mut mytuple, true));
}

#[test]
pub fn test_f2() {
    let mut s_f1: [u32; 6] = [23, 25, 19, 20, 30, 40];
    assert_eq!(&mut 30, p24::refs::f2(&mut s_f1, 5));
    assert_eq!(&mut 19, p24::refs::f2(&mut s_f1, 3));
}

#[test]
pub fn test_f3() {
    let mut s_f1: [u32; 6] = [23, 25, 19, 20, 30, 40];
    assert_eq!(&mut 25, p24::refs::f3(&mut s_f1, 5));
    assert_eq!(&mut 20, p24::refs::f3(&mut s_f1, 3));
}

#[test]
pub fn test_f4() {
    let mut s_f1: [u32; 6] = [23, 25, 19, 20, 30, 40];
    assert_eq!(&mut 20, p24::refs::f3(&mut s_f1, 3));

    let mut mysl: [u32; 10] = [23, 25, 19, 20, 30, 40, 50, 70, 45, 80];
    let test_vec: Vec<[u32; 4]> = vec![[23, 25, 19, 20], [30, 40, 50, 70]];
    assert_eq!(test_vec, p24::refs::f4(&mut mysl));
}
