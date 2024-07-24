use std::fmt::Write;

use rari_types::globals::deny_warnings;
use rari_types::RariEnv;
use tracing::{span, warn, Level};

use super::parser::{parse, Token};
use super::templs::{invoke, TemplatedOrSidebar};
use crate::error::DocError;

pub struct Rendered {
    pub content: String,
    pub templs: Vec<String>,
    pub sidebars: Vec<&'static str>,
}

pub fn render(env: &RariEnv, input: &str) -> Result<Rendered, DocError> {
    let tokens = parse(input)?;
    render_tokens(env, tokens, input)
}
fn encode_ref(index: usize, out: &mut String) -> Result<(), DocError> {
    Ok(write!(out, "!::::{index}::::!",)?)
}

pub fn render_and_decode_ref(env: &RariEnv, input: &str) -> Result<String, DocError> {
    let Rendered {
        content, templs, ..
    } = render(env, input)?;
    decode_ref(&content, &templs)
}

pub(crate) fn decode_ref(input: &str, templs: &[String]) -> Result<String, DocError> {
    let mut decoded = String::with_capacity(input.len());
    if !input.contains("!::::") {
        return Ok(input.to_string());
    }
    let mut frags = vec![];
    for frag in input.split("!::::") {
        let has_ks = frag.contains("::::!");
        for (i, sub_frag) in frag.splitn(2, "::::!").enumerate() {
            if i == 0 && has_ks {
                frags.push(sub_frag);
                //decode_macro(sub_frag, &mut decoded)?;
            } else {
                //decoded.push_str(strip_escape_residues(sub_frag))
                frags.push(sub_frag)
            }
        }
    }
    for i in 0..frags.len() {
        if i % 2 == 1
            && i < frags.len() + 1
            && frags[i - 1].ends_with("<p>")
            && frags[i + 1].starts_with("</p>")
        {
            frags[i - 1] = frags[i - 1].strip_suffix("<p>").unwrap();
            frags[i + 1] = frags[i + 1].strip_prefix("</p>").unwrap();
        }
    }

    for (i, frag) in frags.iter().enumerate() {
        if i % 2 == 1 {
            let index = frag.parse::<usize>()?;
            if let Some(templ) = templs.get(index) {
                decoded.push_str(templ);
            } else {
                return Err(DocError::InvalidTemplIndex(index));
            };
        } else {
            decoded.push_str(frag)
        }
    }

    Ok(decoded)
}
pub fn render_tokens(env: &RariEnv, tokens: Vec<Token>, input: &str) -> Result<Rendered, DocError> {
    let mut templs = vec![];
    let mut sidebars = vec![];
    let mut out = String::with_capacity(input.len());
    for token in tokens {
        match token {
            Token::Text(text) => {
                let slice = &input[text.start..text.end];
                let mut last = 0;
                for (i, _) in slice.match_indices("\\{{") {
                    out.push_str(&slice[last..i]);
                    last = i + 1;
                }
                out.push_str(&slice[last..])
            }
            Token::Macro(mac) => {
                let ident = &mac.ident;
                let span = span!(Level::ERROR, "templ", "{}", &ident);
                let _enter = span.enter();
                match invoke(env, &mac.ident, mac.args) {
                    Ok(TemplatedOrSidebar::Templeated(rendered)) => {
                        encode_ref(templs.len(), &mut out)?;
                        templs.push(rendered)
                    }
                    Ok(TemplatedOrSidebar::Sidebar(sidebar)) => sidebars.push(sidebar),
                    Err(e) if deny_warnings() => return Err(e),
                    Err(e) => {
                        warn!("{e}");
                        writeln!(&mut out, "{e}")?;
                    }
                };
            }
        }
    }
    Ok(Rendered {
        content: out,
        templs,
        sidebars,
    })
}
