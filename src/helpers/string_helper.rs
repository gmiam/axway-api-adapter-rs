pub trait StringExtensions {
    fn remove_quotes(&self) -> Self;
}

impl StringExtensions for String {
    fn remove_quotes(&self) -> Self {
        let mut content = self.to_string();
        if content.starts_with('"') && content.ends_with('"') {
            content.pop();
            content.remove(0);
        }
        content
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_quotes() {
        let to_test = vec![
            r#""foo""#.to_string(),
            r#""""#.to_string(),
            r#""foooooooooooooooooo""#.to_string(),
            r#""foo"#.to_string(),
            r#"foo""#.to_string(),
            r#""foo"bar""#.to_string(),
            r#""foo"bar"#.to_string(),
        ];
        let expected = vec![
            "foo",
            "",
            "foooooooooooooooooo",
            "\"foo",
            "foo\"",
            "foo\"bar",
            "\"foo\"bar",
        ];
        to_test.into_iter().enumerate()
            .for_each(|(key,value)| assert_eq!(&value.remove_quotes(), expected.get(key).unwrap()))
    }
}