struct lettre{
    message: String
}

impl Drop for lettre{
    fn drop(&mut self){
         println!("lettre detruit {}",self.message)
    }
}
fn main() {
    let a = lettre{message: "ismael".to_string()};
    println!("Hello, world!");

}
