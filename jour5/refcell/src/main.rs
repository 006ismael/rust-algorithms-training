use std::rc::Rc;
use std::cell::RefCell;
fn main() {
    let a = Rc::new(RefCell::new(43));
    let b = Rc::clone(&a);
    let c = Rc::clone(&a);
    {
        *b.borrow_mut() += 50 ;
    }

    println!("Hello, world! {}",c.borrow());
}
