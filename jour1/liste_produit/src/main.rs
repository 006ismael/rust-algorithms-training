pub mod produit;
use crate::produit::Produit;
fn main() {
   let produit : Vec<Produit> = vec![
       Produit::new("coco".to_string(),100_000),
       Produit::new("chocolat".to_string(),15_000),
       Produit::new("pepsi".to_string(),5_000),
       Produit::new("eau".to_string(),1_000),
       Produit::new("livre".to_string(),10_000)
   ];
   let isma : Option<Produit> = Produit::plus_cher(&produit);
  println!("{:?}",isma);
}
