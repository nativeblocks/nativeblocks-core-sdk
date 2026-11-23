use icu_locale_core::Locale;
use icu_plurals::{PluralCategory, PluralRules};

pub(crate) fn category(language_code: &str, number: &str) -> Option<&'static str> {
    let number = number.parse::<i64>().ok()?;
    let mut locale = Locale::try_from_str(&language_code.replace('_', "-")).ok()?;
    locale.id.script = None;
    let rules = PluralRules::try_new_cardinal((&locale).into()).ok()?;
    return Some(name(rules.category_for(number)));
}

fn name(category: PluralCategory) -> &'static str {
    return match category {
        PluralCategory::Zero => "zero",
        PluralCategory::One => "one",
        PluralCategory::Two => "two",
        PluralCategory::Few => "few",
        PluralCategory::Many => "many",
        PluralCategory::Other => "other",
    };
}
