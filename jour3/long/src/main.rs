fn long<'a>(a: &'a str,b: &'a str) -> &'a str{
    if a.len()>b.len(){
        a
    }
    else{
        b
    }
}
fn main() {
    println!("Hello, world!");
    assert_eq!(long("isma","qwerty"),"isma");
}
