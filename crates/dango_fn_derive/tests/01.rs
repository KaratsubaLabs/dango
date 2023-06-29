use dango_fn::Func;
use dango_fn_derive::Function;
use serde_json::{json, Map, Value};

#[derive(Function)]
struct WeatherFn {
    /// description
    location: String,
}

fn main() {
    println!("{}", WeatherFn::schema());
}
