#[derive(Debug)]
pub struct BoxedValue<T>{
   pub prix: T
}

impl <T: Default + PartialOrd> BoxedValue<T>{
    pub fn new (prix: T) -> Result<Self,String>{
        if prix < T::default(){
            return Err(format!("prix incorrect") );
        }

        Ok(Self{
            prix
        })
    }
}