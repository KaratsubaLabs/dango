use serde_json::Value;

pub trait Function {
    /// Get json value to pass to openai api
    fn schema() -> Value;
}
