use rari_templ_func::rari_f;

use crate::error::DocError;

#[rari_f]
pub fn secure_context_inline() -> Result<String, DocError> {
    let label = rari_l10n::l10n_json_data("Template", "secure_context_label", env.locale)?;
    let copy = rari_l10n::l10n_json_data("Template", "secure_context_inline_copy", env.locale)?;

    Ok(write_inline_label(label, copy, "secure"))
}

#[rari_f]
pub fn readonly_inline() -> Result<String, DocError> {
    let copy = rari_l10n::l10n_json_data("Template", "readonly_badge_title", env.locale)?;
    let label = rari_l10n::l10n_json_data("Template", "readonly_badge_abbreviation", env.locale)?;

    Ok(write_inline_label(label, copy, "readonly"))
}

pub fn write_inline_label(label: &str, copy: &str, typ: &str) -> String {
    [
        r#"<span class="badge inline "#,
        typ,
        r#"" title=""#,
        &html_escape::encode_double_quoted_attribute(copy),
        r#"">"#,
        label,
        "</span>",
    ]
    .join("")
}
