use std::collections::HashMap;

use crate::feature::localization::domain::plural;
use crate::library::template::{self, Template};

const LOCAL_PARAM: &str = "local_param";
const OTHER: &str = "other";

pub(crate) fn format(
    translation: &str,
    values: &HashMap<String, String>,
    language_code: &str,
) -> String {
    if !template::has(translation, LOCAL_PARAM) {
        return translation.to_string();
    }
    return Template::new()
        .register_nested(LOCAL_PARAM, |body| fill(body, values, language_code))
        .resolve(translation);
}

fn fill(inner: &str, values: &HashMap<String, String>, language_code: &str) -> Option<String> {
    let parts = template::split_top(inner, b'|');
    let name = parts[0].trim();
    if !template::is_name(name) {
        return None;
    }
    let value = values.get(name)?;
    if parts.len() == 1 {
        return Some(value.clone());
    }

    let number = value.trim();
    let mut exact = None;
    let mut forms = Vec::with_capacity(parts.len() - 1);
    for part in &parts[1..] {
        match part.trim_start().strip_prefix('=') {
            Some(rest) => {
                let (expected, text) = rest.split_once('=')?;
                if exact.is_none() && same_number(expected.trim(), number) {
                    exact = Some(text);
                }
            }
            None => {
                let (category, text) = part.split_once('=')?;
                forms.push((category.trim(), text));
            }
        }
    }
    let form = match exact {
        Some(form) => form,
        None => {
            let category = plural::category(language_code, number).unwrap_or(OTHER);
            pick(&forms, category).or_else(|| pick(&forms, OTHER))?
        }
    };
    return Some(fill_form(form, number, values));
}

fn fill_form(form: &str, number: &str, values: &HashMap<String, String>) -> String {
    return Template::new()
        .register(LOCAL_PARAM, |name| {
            let name = name.trim();
            return values
                .get(name)
                .filter(|_| template::is_name(name))
                .cloned();
        })
        .on_text(|text, filled| push_with_number(text, number, filled))
        .resolve(form);
}

fn push_with_number(text: &str, number: &str, filled: &mut String) {
    let mut rest = text;
    while let Some(index) = rest.find('#') {
        filled.push_str(&rest[..index]);
        if rest[index + 1..].starts_with('#') {
            filled.push('#');
            rest = &rest[index + 2..];
        } else {
            filled.push_str(number);
            rest = &rest[index + 1..];
        }
    }
    filled.push_str(rest);
}

fn same_number(expected: &str, number: &str) -> bool {
    return match (expected.parse::<f64>(), number.parse::<f64>()) {
        (Ok(expected), Ok(number)) => expected.is_finite() && expected == number,
        _ => false,
    };
}

fn pick<'a>(forms: &[(&str, &'a str)], category: &str) -> Option<&'a str> {
    return forms
        .iter()
        .find(|(name, _)| *name == category)
        .map(|(_, text)| *text);
}
