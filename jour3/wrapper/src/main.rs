pub mod boxed;
use crate::boxed::BoxedValue;
fn main() -> Result<(),String> {
    println!("Hello, world!");
    let isma : BoxedValue<f32> = BoxedValue::<f32>::new(-1000.0)?;
        println!("{:?}",isma);
        Ok(())
}
