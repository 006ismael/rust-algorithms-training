use std::mem::transmute;
fn main() {
    println!("Hello, world!");
    let isma: u32 = 50;
    unsafe{
      let adress: [u8;4] = transmute(isma) ;
      println!("{:?}",adress);
    }
}
