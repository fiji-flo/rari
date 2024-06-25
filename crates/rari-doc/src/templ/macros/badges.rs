use rari_templ_func::rari_f;
use rari_types::locale::Locale;

use crate::error::DocError;

#[rari_f]
pub fn experimental() -> Result<String, DocError> {
    let mut out = String::new();
    write_experimental(&mut out, env.locale)?;
    Ok(out)
}

#[rari_f]
pub fn non_standard() -> Result<String, DocError> {
    let mut out = String::new();
    write_non_standard(&mut out, env.locale)?;
    Ok(out)
}

#[rari_f]
pub fn deprecated() -> Result<String, DocError> {
    let mut out = String::new();
    write_deprecated(&mut out, env.locale)?;
    Ok(out)
}

pub fn write_experimental(out: &mut impl std::fmt::Write, locale: Locale) -> std::fmt::Result {
    let title = rari_l10n::l10n("experimental_badge_title", locale);
    let abbreviation = rari_l10n::l10n("experimental_badge_abbreviation", locale);

    write_badge(out, title, abbreviation, "experimental")
}

pub fn write_non_standard(out: &mut impl std::fmt::Write, locale: Locale) -> std::fmt::Result {
    let title = rari_l10n::l10n("non_standard_badge_title", locale);
    let abbreviation = rari_l10n::l10n("non_standard_badge_abbreviation", locale);

    write_badge(out, title, abbreviation, "nonstandard")
}

pub fn write_deprecated(out: &mut impl std::fmt::Write, locale: Locale) -> std::fmt::Result {
    let title = rari_l10n::l10n("deprecated_badge_title", locale);
    let abbreviation = rari_l10n::l10n("deprecated_badge_abbreviation", locale);

    write_badge(out, title, abbreviation, "deprecated")
}

pub fn write_badge(
    out: &mut impl std::fmt::Write,
    title: &str,
    abbreviation: &str,
    typ: &str,
) -> std::fmt::Result {
    write!(
        out,
        r#"<abbr class="icon icon-{typ}" title="{title}">
<span class="visually-hidden">{abbreviation}</span>
</abbr>"#
    )
}
