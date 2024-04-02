 pub fn reply(message: &str) -> &str {
  if message.is_empty() {
    "Fine. Be that way!"
  } else if message.is_uppercase() {
    if message.ends_with("?") {
      "Calm down, I know what I'm doing!"
    } else {
      "Whoa, chill out!"
    }
  } else if message.ends_with("?") {
    "Sure."
  } else {
    "Whatever."
  }
}
