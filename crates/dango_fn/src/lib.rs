use serde_json::Value;

pub trait Func {
    /// Get json value to pass to openai api
    fn schema() -> Value;
}
