
Inspired by rocket, each openai function is an actual rust function.

```rust

#[derive(DangoArg)]
pub struct Weather {
    #[dango(description = "the location to get the weather in")] 
    location: String,
}

#[dango(name = "weather", description = "get the weather in a given city")]
fn weather(arg: Weather) {

}

client.register(weather);

```
