pub mod produit;
use crate::produit::CartItem;

fn main() {
    let items: Vec<CartItem> = vec![
      CartItem::new("chocolate".to_string(),130.0,1),
      CartItem::new("huile".to_string(),1300.0,1),
      CartItem::new("coco".to_string(),530.0,1),
      CartItem::new("tomate".to_string(),30.0,1),
    ];
    let isma = 130.0+1300.0+530.0+30.0;
    assert_eq!(CartItem::total_panier(&items),isma);
}
