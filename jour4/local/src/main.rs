fn long(nom: &str) -> usize{
    nom.len()
}
fn main() {
    let isma : &'static str = "ismael";
    println!("Hello, world!");
    let boula: usize = long(isma);
    println!("{}",boula);
}
