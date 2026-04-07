#[derive(Debug,Clone)]
pub struct Produit {
  pub nom: String,
  pub prix: i64
}

impl Produit{
   pub fn plus_cher(produits : &[Self])-> Option<Self>{
       produits.iter()
               .max_by(|a,b| a.prix.cmp(&b.prix))
               .cloned()
   }
   pub fn new (nom: String ,prix: i64)-> Self {
      Self{
         nom ,
         prix
      }
   }
   
} 