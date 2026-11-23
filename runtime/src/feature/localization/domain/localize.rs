use std::collections::HashMap;

use crate::library::template::{self, Template};

const LOCAL: &str = "local";

pub(crate) fn has_calls(text: &str) -> bool {
    return template::has(text, LOCAL);
}

pub(crate) fn localize(
    text: &str,
    translate: impl Fn(&str, &HashMap<String, String>) -> Option<String>,
) -> String {
    return Template::new()
        .register(LOCAL, |call| {
            let (key, values) = parse_call(call)?;
            return Some(translate(key, &values).unwrap_or_else(|| key.to_string()));
        })
        .resolve(text);
}

fn parse_call(call: &str) -> Option<(&str, HashMap<String, String>)> {
    let Some(open) = call.find('(') else {
        return template::is_key(call).then(|| (call, HashMap::new()));
    };
    let key = &call[..open];
    let inner = call[open + 1..].strip_suffix(')')?;
    if !template::is_key(key) {
        return None;
    }

    let mut values = HashMap::new();
    if inner.trim().is_empty() {
        return Some((key, values));
    }
    for arg in split_args(inner) {
        let (name, value) = arg.split_once('=')?;
        let name = name.trim();
        if !template::is_name(name) {
            return None;
        }
        values.insert(name.to_string(), value.trim().to_string());
    }
    return Some((key, values));
}

fn split_args(inner: &str) -> Vec<&str> {
    let mut args = Vec::new();
    let mut start = 0;
    for (index, _) in inner.match_indices(',') {
        if starts_argument(&inner[index + 1..]) {
            args.push(&inner[start..index]);
            start = index + 1;
        }
    }
    args.push(&inner[start..]);
    return args;
}

fn starts_argument(rest: &str) -> bool {
    return rest
        .split_once('=')
        .is_some_and(|(name, _)| template::is_name(name.trim()));
}
