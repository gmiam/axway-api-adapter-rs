pub trait StringExtensions {
    fn remove_quotes(&self) -> Self;
}

impl StringExtensions for String {
    fn remove_quotes(&self) -> Self {
        let mut content = self.to_string();
        if content.chars().next() == Some('"') && content.chars().last() == Some('"') {
            content.pop();
            content.remove(0);
        }
        content
    }
}
