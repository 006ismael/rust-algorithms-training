pub struct CartItem {
    pub nom: String,
    pub price: f32,
    pub quantity: u32,
}
pub struct Order {
    items: Vec<CartItem>,
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
    pub fn appliquer_reduction(total: f32) -> f32{
     let reduction: f32 =   match total {
        x if x >= 100.0 => x - ((x*20.0)/100.0),
        x if x >= 50.0  => x - ((x*10.0)/100.0),
        x   => x
        };
        reduction
    }
}
impl  Order{
  pub fn new(items: Vec<CartItem>) -> Self{
    Self{
        items
    }
  }
  pub fn valider_commande(&self) -> bool{
   let commande: bool =   match &self.items {
        x if CartItem::total_panier(&x) > 0.0 => true,
        _ =>false
      };
      commande
  }
}