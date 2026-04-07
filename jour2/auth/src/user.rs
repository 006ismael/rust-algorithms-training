pub struct User{
   pub email:    String,
   pub password: String,
}
pub enum Role{
    Admin,
    User  
} 
impl User{
    pub fn new(email: String, password: String) -> Self{
        Self{
            email,
            password
        }
    }
    pub fn login(liste: &[User], email: String, password: String) -> bool{
       for user in liste.iter(){
    if user.email == email && user.password == password {
        return true;
    }
    }
    false
}
}
pub fn peut_supprimer(role: Role) -> bool{
  match role {
    Role::Admin => { true },
    Role::User  => { false }
  }
}
