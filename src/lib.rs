use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub struct CustomData {
    pub command: Option<String>,
    pub data: HashMap<String, String>,
}

pub fn parse_custom_format(input: &str) -> Result<CustomData, &'static str> {
    let mut data = HashMap::new();
    let mut command = None;

    let iter = input.chars().peekable();
    let mut word = Vec::new();
    let mut words = Vec::new();
    let mut old_key = None;
    for c in iter {
        if c == ' ' {
            if !word.is_empty() {
                let word_string = word.iter().collect();
                word.clear();
                if command.is_none() && data.is_empty() {
                    command = Some(word_string);
                } else {
                    words.push(word_string);
                }
            }
            continue;
        }

        if c == ':' {
            let key = if !word.is_empty() {
                let s: String = word.iter().collect();
                word.clear();
                s
            } else if let Some(key) = words.pop() {
                key
            } else {
                return Err("String cannot start with \n:\n");
            };
            if let Some(k) = old_key {
                data.insert(k, words.join(" "));
                words.clear();
            }
            old_key = Some(key);
        } else {
            word.push(c);
        }
    }

    if !word.is_empty() {
        let word_string = word.iter().collect();
        word.clear();
        words.push(word_string);
    }

    if let Some(k) = old_key {
        data.insert(k, words.join(" "));
        words.clear();
    }

    Ok(CustomData { command, data })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "main minWidth: 3 maxWidth: 15 seed: 200";
        let parsed = parse_custom_format(input);
        let mut expected = HashMap::new();
        expected.insert("minWidth".to_string(), "3".to_string());
        expected.insert("maxWidth".to_string(), "15".to_string());
        expected.insert("seed".to_string(), "200".to_string());
        assert_eq!(parsed, Ok(CustomData { command: Some("main".to_string()), data: expected }));
    }
}
