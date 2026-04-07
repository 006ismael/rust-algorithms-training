use num::Float;
fn min<T> (liste: &[T]) -> Option<&T>
    where 
    T:PartialOrd,
    T:Float
{
    liste.iter().filter(|x| !(*x).is_nan()).reduce(|a,b| {
        if a.partial_cmp(&b) == Some(std::cmp::Ordering::Less){
            a
        }
        else{
            b
        }
    }
        )
}
fn main() {
    println!("Hello, world!");
    let isma = vec![
      1.0,56.5,67.90,45.78,90.9
    ];
   let boula = min(&isma);
   println!("{:?}",boula);
}
