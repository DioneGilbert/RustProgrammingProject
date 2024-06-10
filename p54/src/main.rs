// use crypto::aes;
use openssl::aes;
use ring::aead::quic::AES_128;

fn main() {
    println!("Hello world");
    //p54::aes_ni::aes::;
    //let key = aes::Key::generate();
    // let key = aes::Key::generate();
    let key = openssl::cipher::Cipher::aes_128_ctr();
    println!("{:?}", key1)
}
