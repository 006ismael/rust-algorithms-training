use std::rc::Rc;
fn main() {
    println!("Hello, world!");
    let a: Vec<i32> = vec![32,56,89];
    let isma = Rc::<Vec<i32>>::new(a);
    let b = Rc::clone(&isma);
    let c = Rc::clone(&isma);
        println!("{:?}",isma);
         println!("{:?}",b);
          println!("{:?}",c);
}
