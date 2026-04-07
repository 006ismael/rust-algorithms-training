pub fn fois2(nombre: Option<i32>)-> i32 {
 match nombre{
    Some(x) => return x*2,
    None    => return 0
 }
}
fn main() {
    let num: Option<i32> = None;
        assert_eq!(fois2(num),1); 
}
