pub struct Produit{
  pub id: u32,
  pub price: f32
}
pub struct CartItem{
 pub product_id: u32,
 pub quantity: u32

}
pub fn calculer_total(cart: Vec<CartItem>, produits: Vec<Produit>) -> f32{
  let mut  multiple: Vec<f32>  = vec![];
     
        for cart_produit in &cart{

          for produit in &produits{

          if produit.id == cart_produit.product_id{
            if cart_produit.quantity > 0{
            multiple.push(produit.price * (cart_produit.quantity as f32));
            }
            else {
              println!("stock de {} epuisser",cart_produit.product_id);
            }
          }
        }

     }
     multiple.iter().sum()
}