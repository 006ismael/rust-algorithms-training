static mut COUNTER: i32 = 10;
fn main() {
    println!("Hello, world!");
    unsafe {
        fn incremente(count: &mut i32){
            *count += 10
        }
    incremente(&mut *(&raw mut COUNTER));
    incremente(&mut *(&raw mut COUNTER));
    incremente(&mut *(&raw mut COUNTER));
       println!("{}",*(&raw const COUNTER));
    }
   
    
}
