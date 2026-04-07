 pub struct Produit{
   pub name: String,
   pub stock: u32
 }
 
 
 
 impl Produit{
    pub fn new(name: String,stock: u32) -> Self{
        Self{
            name,
            stock
        }
    }
 pub fn acheter( &mut self, quantite: u32) -> bool{
       if self.stock >= quantite{
          self.stock -=  quantite;
        return true;
       }
       false
   }
}