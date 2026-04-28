use std::collections::HashMap;

pub(crate) struct ParseResult<'a> {
    pub(crate) map: HashMap<&'a str, &'a str>,
    pub(crate) errors_found: bool,
    pub(crate) missing_quotes_found: bool,
}

pub(crate) fn parse_delimited_string(val: &str) -> ParseResult {
    let elements = val.trim().split(';');
    let mut map = HashMap::new();
    let mut errors_found = false;
    let mut missing_quotes_found = false;
    for element in elements {
        if let (Some((key, value)), missing_quotes) = parse_key_value(element) {
            map.insert(key, value);
            if missing_quotes {
                missing_quotes_found = true;
            }
        } else {
            errors_found = true;
        }
    }
    ParseResult {
        map,
        missing_quotes_found,
        errors_found,
    }
}

fn parse_key_value(val: &str) -> (Option<(&str, &str)>, bool) {
    let kv: Vec<_> = val.splitn(2, '=').collect();
    if kv.len() != 2 {
        return (None, false);
    }
    let (key, mut value) = (kv[0].trim(), kv[1].trim());
    let mut missing_quotes = false;
    if value.starts_with('\'') && value.ends_with('\'') && value.len() > 1 {
        value = &value[1..value.len() - 1];
    } else {
        missing_quotes = true;
    }
    (Some((key, value)), missing_quotes)
}

pub(crate) fn parse_value_if_valid(s: &str) -> Option<String> {
    let s = if s.ends_with(';') {
        s.trim_end_matches(';')
    } else {
        s
    };
    if let (Some((_, s)), _) = parse_key_value(s) {
        Some(s.to_string())
    } else {
        None
    }
}
