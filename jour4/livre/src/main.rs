mod livre;
use crate::livre::Livre;
fn main() {
    println!("Hello, world!");
  let isma: Box<Livre> = Box::new(Livre{id: 1,nom: "l autoroute du milionnaire".to_string(),stock: 200});
    println!("{:?}",isma)
}
