pub fn reply(message: &str) -> &str {
    let message = message.trim(); // Trim whitespace characters
    
    if message.is_empty() {
        "Fine. Be that way!"
    } else if message.chars().all(|c| c.is_uppercase()) && message.ends_with('?') {
        "Calm down, I know what I'm doing!"
    } else if message.ends_with('?') {
        "Sure."
    } else if message.chars().all(|c| !c.is_lowercase()) {
        "Whoa, chill out!"
    } else {
        "Whatever."
    }
}
