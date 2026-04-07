fn transf(chaine: Option<String>) -> Option<usize>{
    chaine.and_then(|c| {
      if c.is_empty() == false {
         return Some(c.len());
      }
       else {
        return None;
      }
    })
}
fn main() {
    println!("Hello, world!");
    let x =transf(Some("".to_string()));
   println!("{:?}",x) ;
}
