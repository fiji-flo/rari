use std::collections::HashMap;
use std::sync::LazyLock;

use css_syntax::syntax::{write_formal_syntax, CssType, LinkedToken};
use rari_templ_func::rari_f;
use tracing::error;

use crate::error::DocError;

static TOOLTIPS: LazyLock<HashMap<LinkedToken, String>> = LazyLock::new(|| {
    [(LinkedToken::Asterisk, "Asterisk: the entity may occur zero, one or several times".to_string()),
    (LinkedToken::Plus, "Plus: the entity may occur one or several times".to_string()),
    (LinkedToken::QuestionMark, "Question mark: the entity is optional".to_string()),
    (LinkedToken::CurlyBraces, "Curly braces: encloses two integers defining the minimal and maximal numbers of occurrences of the entity, or a single integer defining the exact number required".to_string()),
    (LinkedToken::HashMark, "Hash mark: the entity is repeated one or several times, each occurence separated by a comma".to_string()),
    (LinkedToken::ExclamationPoint,"Exclamation point: the group must produce at least one value".to_string()),
    (LinkedToken::Brackets, "Brackets: enclose several entities, combinators, and multipliers to transform them as a single component".to_string()),
    (LinkedToken::SingleBar, "Single bar: exactly one of the entities must be present".to_string()),
    (LinkedToken::DoubleBar, "Double bar: one or several of the entities must be present, in any order".to_string()),
    (LinkedToken::DoubleAmpersand, "Double ampersand: all of the entities must be present, in any order".to_string())].into_iter().collect()
});

#[rari_f]
pub fn csssyntax() -> Result<String, DocError> {
    let page_type = env.page_type;
    let mut slug_rev_iter = env.slug.rsplitn(3, '/');
    let name = slug_rev_iter.next().unwrap();
    let typ = match page_type {
        rari_types::fm_types::PageType::CssAtRule => CssType::AtRule(name),
        rari_types::fm_types::PageType::CssAtRuleDescriptor => {
            CssType::AtRuleDescriptor(name, slug_rev_iter.next().unwrap())
        }
        rari_types::fm_types::PageType::CssCombinator => todo!(),
        rari_types::fm_types::PageType::CssFunction => CssType::Function(name),
        rari_types::fm_types::PageType::CssKeyword => todo!(),
        rari_types::fm_types::PageType::CssMediaFeature => todo!(),
        rari_types::fm_types::PageType::CssModule => todo!(),
        rari_types::fm_types::PageType::CssProperty => CssType::Property(name),
        rari_types::fm_types::PageType::CssPseudoClass => todo!(),
        rari_types::fm_types::PageType::CssPseudoElement => todo!(),
        rari_types::fm_types::PageType::CssSelector => todo!(),
        rari_types::fm_types::PageType::CssShorthandProperty => CssType::ShorthandProperty(name),
        rari_types::fm_types::PageType::CssType => CssType::Type(name),
        _ => {
            error!("No Css Page: {}", env.slug);
            return Err(DocError::CssPageTypeRequired);
        }
    };

    Ok(write_formal_syntax(
        typ,
        env.locale.as_url_str(),
        &format!(
            "/{}/docs/Web/CSS/Value_definition_syntax",
            env.locale.as_url_str()
        ),
        &TOOLTIPS,
    )?)
}
