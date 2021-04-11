/// Enquotes a string in a safe way
pub fn enquote<S: ToString>(value: S) -> String {
    let value = value.to_string();
    format!("\"{}\"", value.replace("\"", "\\\""))
}
