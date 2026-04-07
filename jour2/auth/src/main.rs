mod user;
use crate::user::{User,Role,peut_supprimer};

fn main() {
    let utilisateur: Vec<User> = vec![
     User::new("ismael".to_string(),"ismael".to_string()),
     User::new("ismael1".to_string(),"ismael1".to_string()),
     User::new("ismael2".to_string(),"ismael2".to_string()),
     User::new("ismael3".to_string(),"ismael3".to_string()),
    ];
   assert_eq!(User::login(&utilisateur,"ismael".to_string(),"ismael".to_string()),true) ;
  let isma: Role = Role::Admin;
  let is: Role = Role::User;
  assert_eq!(peut_supprimer(isma),true);
  assert_eq!(peut_supprimer(is),true);
}
