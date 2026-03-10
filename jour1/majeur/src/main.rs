pub mod person;
use crate::person::User;
fn main() {
    let personnage: Vec<User> = vec![
      User::new("ismael".to_string(),15),
      User::new("ibrahima".to_string(),18),
      User::new("dioman".to_string(),16),
      User::new("abdoulaye".to_string(),35),
      User::new("Maiga".to_string(),25),
    ];
    let isma: Vec<User> = User::utilisateurs_majeurs(&personnage);
        println!("{:?}",isma)
}
