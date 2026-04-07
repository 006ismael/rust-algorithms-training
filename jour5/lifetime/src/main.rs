#[derive(Debug)]
struct Highlight<'a>{
    id : i32,
    livre: &'a String
}
fn main() {
    println!("Hello, world!");
    let lettre = "ismael".to_string();
   let isma: Highlight = Highlight {id: 2, livre: &lettre};
   println!("{:?}",isma);
}
