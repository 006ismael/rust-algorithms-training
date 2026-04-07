mod produit;
use crate::produit::{Produit,CartItem,calculer_total};


fn main(){
  let vec_produit: Vec<Produit> =vec![
    Produit {id: 1,price: 200.50},
    Produit {id: 2,price: 50.90},
    Produit {id: 3,price: 40.55},
    Produit {id: 4,price: 10.64}
    ] ;
     let vec_cart: Vec<CartItem> =vec![
    CartItem {product_id: 1,quantity: 20},
    CartItem {product_id: 2,quantity: 50},
    CartItem {product_id: 3,quantity: 4},
    CartItem {product_id: 4,quantity: 0}
    ] ;
    println!("{}",calculer_total(vec_cart,vec_produit))
}