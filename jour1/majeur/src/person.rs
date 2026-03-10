#[derive(Debug,Clone)]
pub struct User{
    pub nom: String,
    pub age: u32
}
impl User{
    pub fn new(nom: String ,age: u32) -> Self{
        Self{
            nom,
            age
        }
    }
    
    pub fn utilisateurs_majeurs(users: &[Self]) -> Vec<Self>{
               let tab_user: Vec<User> =  users.iter()
                                                .filter(|x| x.age>=18)
                                                .cloned()
                                                .collect();
         
        tab_user
    }

}