use rari_templ_func::rari_f;

use crate::error::DocError;

#[rari_f]
pub fn web_ext_examples(level: Option<String>) -> Result<String, DocError> {
    let mut split = env.slug.rsplitn(3, '/');
    let leaf = split.next();
    let parent = split.next();
    let mut out = String::new();
    Ok(Default::default())
}
