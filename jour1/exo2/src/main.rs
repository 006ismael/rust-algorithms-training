//cette fonction permet de determiner le nombre le plus grand entre trois nombre 
fn max3(a: i32, b: i32, c: i32) -> i32{
  let mut max = a;
  if b > max { max = b}
  if c > max { max = c }
  max
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn f1(){
     assert_eq!(max3(12,50,90),90)
    }
     #[test]
    fn f2(){
     assert_eq!(max3(1100,110,9),1100)
    }
     #[test]
    fn f3(){
     assert_eq!(max3(12,500,90),500)
    }
     #[test]
    fn f4(){
     assert_eq!(max3(50,50,90),90)
    }
     #[test]
    fn f5(){
     assert_eq!(max3(12,50,50),50)
    }
     #[test]
    fn f6(){
     assert_eq!(max3(50,50,50),50)
    }
      #[test]
    fn f7(){
     assert_eq!(max3(12,90,90),90)
    }
     #[test]
    fn f8(){
     assert_eq!(max3(100,100,90),100)
    }
    #[test]
     fn f9(){
     assert_eq!(max3(90,100,100),100)
    }
    #[test]
     fn f10(){
     assert_eq!(max3(100,90,100),100)
    }
}
