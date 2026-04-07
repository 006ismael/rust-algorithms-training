//cette fontion permet de voir si un nombre est pair ou impair si le nombre est pair il return true dans le cas contraire false
fn est_pair(n : i32) -> bool{
  n % 2 == 0
}
#[cfg(test)]
mod tests{
use super::*;
#[test]
fn test_si_5_est_pair(){
    assert_eq!(est_pair(5),false)
}
#[test]
fn test_si_6_est_pair(){
    assert_eq!(est_pair(6),true)
}
#[test]
fn test_si_7_est_pair(){
    assert_eq!(est_pair(7),false)
}
#[test]
fn test_si_8_est_pair(){
    assert_eq!(est_pair(8),true)
}
}
