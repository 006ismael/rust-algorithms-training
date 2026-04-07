pub mod produit;
use crate::produit::Produit;

fn main() {
let mut isma: Produit = Produit::new("coco".to_string(),1000);
assert_eq!(isma.acheter(500),true);
assert_eq!(isma.acheter(600),true);
}
