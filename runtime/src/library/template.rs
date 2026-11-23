const OPEN: &str = "{{";
const CLOSE: &str = "}}";

type Handler<'a> = Box<dyn FnMut(&str) -> Option<String> + 'a>;
type TextHandler<'a> = Box<dyn FnMut(&str, &mut String) + 'a>;

struct Pattern<'a> {
    open: String,
    nested: bool,
    handler: Handler<'a>,
}

pub(crate) struct Template<'a> {
    patterns: Vec<Pattern<'a>>,
    on_text: Option<TextHandler<'a>>,
}

impl<'a> Template<'a> {
    pub(crate) fn new() -> Self {
        return Self {
            patterns: Vec::new(),
            on_text: None,
        };
    }

    pub(crate) fn register(
        self,
        prefix: &str,
        handler: impl FnMut(&str) -> Option<String> + 'a,
    ) -> Self {
        return self.add(prefix, false, handler);
    }

    pub(crate) fn register_nested(
        self,
        prefix: &str,
        handler: impl FnMut(&str) -> Option<String> + 'a,
    ) -> Self {
        return self.add(prefix, true, handler);
    }

    pub(crate) fn on_text(mut self, handler: impl FnMut(&str, &mut String) + 'a) -> Self {
        self.on_text = Some(Box::new(handler));
        return self;
    }

    pub(crate) fn resolve(mut self, text: &str) -> String {
        let mut resolved = String::with_capacity(text.len());
        let mut rest = text;
        while let Some(start) = rest.find(OPEN) {
            let candidate = &rest[start..];
            let Some(index) = self
                .patterns
                .iter()
                .position(|pattern| candidate.starts_with(&pattern.open))
            else {
                self.push_text(&rest[..start + 1], &mut resolved);
                rest = &rest[start + 1..];
                continue;
            };
            let pattern = &mut self.patterns[index];
            let body = &candidate[pattern.open.len()..];
            let end = if pattern.nested {
                close_nested(body, &pattern.open)
            } else {
                body.find(CLOSE)
            };
            let Some(end) = end else {
                break;
            };
            let used = start + pattern.open.len() + end + CLOSE.len();
            let replacement = (pattern.handler)(&body[..end]);
            self.push_text(&rest[..start], &mut resolved);
            match replacement {
                Some(replacement) => resolved.push_str(&replacement),
                None => resolved.push_str(&rest[start..used]),
            }
            rest = &rest[used..];
        }
        self.push_text(rest, &mut resolved);
        return resolved;
    }

    fn add(
        mut self,
        prefix: &str,
        nested: bool,
        handler: impl FnMut(&str) -> Option<String> + 'a,
    ) -> Self {
        self.patterns.push(Pattern {
            open: open_of(prefix),
            nested,
            handler: Box::new(handler),
        });
        return self;
    }

    fn push_text(&mut self, text: &str, resolved: &mut String) {
        match self.on_text.as_mut() {
            Some(on_text) => on_text(text, resolved),
            None => resolved.push_str(text),
        }
    }
}

pub(crate) fn has(text: &str, prefix: &str) -> bool {
    return text.contains(&open_of(prefix));
}

pub(crate) fn reference(prefix: &str, body: &str) -> String {
    return format!("{}{body}{CLOSE}", open_of(prefix));
}

fn open_of(prefix: &str) -> String {
    return format!("{OPEN}{prefix}:");
}

fn close_nested(body: &str, open: &str) -> Option<usize> {
    let bytes = body.as_bytes();
    let mut i = 0;
    while i + 1 < bytes.len() {
        if bytes[i] == b'}' && bytes[i + 1] == b'}' {
            return Some(i);
        }
        if bytes[i] == b'{' && body[i..].starts_with(open) {
            let close = body[i + open.len()..].find(CLOSE)?;
            i += open.len() + close + CLOSE.len();
        } else {
            i += 1;
        }
    }
    return None;
}

pub(crate) fn split_top(text: &str, separator: u8) -> Vec<&str> {
    let bytes = text.as_bytes();
    let mut parts = Vec::new();
    let mut depth = 0usize;
    let mut start = 0;
    let mut i = 0;
    while i < bytes.len() {
        let pair = bytes.get(i + 1).map(|next| (bytes[i], *next));
        match pair {
            Some((b'{', b'{')) => {
                depth += 1;
                i += 2;
                continue;
            }
            Some((b'}', b'}')) if depth > 0 => {
                depth -= 1;
                i += 2;
                continue;
            }
            _ => {}
        }
        if depth == 0 && bytes[i] == separator {
            parts.push(&text[start..i]);
            start = i + 1;
        }
        i += 1;
    }
    parts.push(&text[start..]);
    return parts;
}

pub(crate) fn is_name(name: &str) -> bool {
    return !name.is_empty() && name.chars().all(|c| c.is_alphanumeric() || c == '_');
}

pub(crate) fn is_key(key: &str) -> bool {
    return !key.is_empty()
        && key
            .chars()
            .all(|c| c.is_alphanumeric() || c == '_' || c == '.' || c == '-');
}
