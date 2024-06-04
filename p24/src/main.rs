pub fn f3(s: &mut [i32], length: usize) -> &i32 {
    println!("===========================");
    println!("{:?}", s);
    return &mut s[s.len() - length];
}

fn main() {
    println!("Hello, world!");
    // let mut myslice = [23, 25, 19, 20, 30, 40, 50, 70];
    // let s_v: &mut [i32] = &mut myslice[0..6];
    // println!("{}", s_v.len());
    // println!("{:?}", s_v);
    // println!("{}", s_v[s_v.len() - 5]);
    // // let res: &mut i32 = &mut f3(s_v, 6);
    // let res: i32 = *f3(s_v, 5);
    // println!("---here---{:?}", res);
    let mut mysl: [u32; 10] = [23, 25, 19, 20, 30, 40, 50, 70, 45, 80];
    // let length = mysl.len() / 4;
    println!("---length = {:?}", mysl);
    // println!("---length = {:?} ", length);
    let test_vec: Vec<[u32; 3]> = vec![[10, 20, 50], [100, 200, 500]];
    println!("---length = {:?} ", test_vec.len());
    println!("--- = {:?} ", test_vec[0]);
    let myvec: Vec<[u32; 4]> = p24::refs::f4(&mut mysl);
    println!("---length = {:?} ", myvec.len());
    println!("---length = {:?} ", myvec);
}
