use std::ptr::read;
fn main() {
    let isma: i32 = 148 ;
    let boula = &isma as *const i32;
    unsafe{

         println!("{:?}",read(boula));
    }
    
}
