/*!n = n * !(n-1) ! 0 | 1 = 1 */
fn factorielle(n: u32) -> u32{
    if n == 0 || n == 1{
        1
    }
    else {
       n * factorielle(n - 1)
    }
}

#[cfg(test)]
mod tests{
    use super::*;
#[test]
fn f1(){
assert_eq!(factorielle(3),6)
}
#[test]
fn f2(){
assert_eq!(factorielle(4),24)
}
#[test]
fn f3(){
assert_eq!(factorielle(5),120)
}
#[test]
fn f4(){
assert_eq!(factorielle(6),720)
}
#[test]
fn f5(){
assert_eq!(factorielle(1),1)
}
#[test]
fn f6(){
assert_eq!(factorielle(0),1)
}
}