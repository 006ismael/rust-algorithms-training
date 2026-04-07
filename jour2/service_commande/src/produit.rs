pub struct Product {
    id: u32,
    price: f32,
    stock: u32,
}

pub struct CartItem {
    product_id: u32,
    quantity: u32,
}

pub struct Order {
    items: Vec<CartItem>,
}


pub fn process_order(order: Order, products: &mut Vec<Product>) -> Result<f32, String>{
        let mut total : f32 = 0.0 ;
            for orders in &order.items{
               let items = products.iter_mut()
               .find(|p|{
                    orders.product_id == p.id
                })
                .ok_or(format!("Produit ID {} non trouvé", item.product_id))?;
            if items.stock < orders.quantity {
                Err(format!("Stock insuffisant pour le produit {}", product.id))
            }
              products.stock -= items.quantity;
              total += product.price * (items.quantity as f32);
            }
     Ok(total)
}