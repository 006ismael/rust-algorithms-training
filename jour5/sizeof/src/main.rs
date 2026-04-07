fn main() {
    println!("Hello, world!");
    assert_eq!(std::mem::size_of::< *const i32>(),std::mem::size_of::< Option<&i32>>());
}
