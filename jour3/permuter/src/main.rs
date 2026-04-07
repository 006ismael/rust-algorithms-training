use std::ptr::{write,read};
fn swap<T>(a: &mut T, b: &mut T){
  let ptr_a = a as *mut T;
  let ptr_b = b as *mut T;

  unsafe{
    let inter = read(ptr_a);
    write(ptr_a,read(ptr_b));
     write(ptr_b,inter);
  }
}
fn main() {
    let mut a: f32 =12.0;
    let mut b: f32 =42.0;
    println!("Hello, world!");
    swap::<f32>(&mut a, &mut b);
        println!("{a},{b}");
}
