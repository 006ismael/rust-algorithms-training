pub struct CartItem {
    pub nom: String,
    pub price: f32,
    pub quantity: u32,
}

impl CartItem{
    pub fn new( nom: String ,price: f32 ,quantity: u32) -> Self{
        Self{
            nom,
            price,
            quantity
        }
    }
    pub fn total_panier(items: &[CartItem]) -> f32{
          let new_items: f32  = items.into_iter()
                .map(|x|{
                    x.price * (x.quantity as f32)
                })
                .sum();
        new_items
    }
}