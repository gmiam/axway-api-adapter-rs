pub fn remove_quotes(mut content: String) -> String {
    if content.chars().next() == Some('"') && content.chars().last() == Some('"') {
        content.pop();
        content.remove(0);
    }
    content
}