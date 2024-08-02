use std::path::{Path, PathBuf};
use std::sync::Arc;

use enum_dispatch::enum_dispatch;
use rari_types::fm_types::{FeatureStatus, PageType};
use rari_types::globals::blog_root;
use rari_types::locale::Locale;
use rari_types::RariEnv;

use super::curriculum::CurriculumPage;
use super::doc::Doc;
use super::dummy::Dummy;
use crate::cached_readers::{blog_from_url, curriculum_from_url};
use crate::docs::blog::BlogPost;
use crate::error::DocError;
use crate::resolve::{strip_locale_from_url, url_path_to_path_buf};
use crate::utils::{locale_and_typ_from_path, root_for_locale};

#[derive(Debug, Clone)]
#[enum_dispatch]
pub enum Page {
    Doc(Arc<Doc>),
    BlogPost(Arc<BlogPost>),
    Dummy(Arc<Dummy>),
    Curriculum(Arc<CurriculumPage>),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum PageCategory {
    Doc,
    BlogPost,
    Dummy,
    Curriculum,
}

impl Page {
    pub fn page_from_url_path(url_path: &str) -> Result<Page, DocError> {
        url_path_to_page(url_path)
    }

    pub fn ignore(url_path: &str) -> bool {
        if url_path == "/discord" {
            return true;
        }
        if url_path == "/en-US/blog/rss.xml" {
            return true;
        }
        if url_path.starts_with("/users/") {
            return true;
        }
        if url_path.starts_with("/en-US/plus") {
            return true;
        }
        if url_path.starts_with("/en-US/play") {
            return true;
        }

        false
    }
    pub fn exists(url_path: &str) -> bool {
        if url_path == "/discord" {
            return true;
        }
        if url_path.starts_with("/users/") {
            return true;
        }
        if url_path.starts_with("/en-US/blog") && blog_root().is_none() {
            return true;
        }
        if url_path.starts_with("/en-US/curriculum") {
            return true;
        }
        if strip_locale_from_url(url_path).1 == "/" {
            return true;
        }

        Page::page_from_url_path(url_path).is_ok()
    }
}

impl PageReader for Page {
    fn read(path: impl Into<PathBuf>) -> Result<Page, DocError> {
        let path = path.into();
        let (_, typ) = locale_and_typ_from_path(&path)?;
        match typ {
            PageCategory::Doc => Doc::read(path),
            PageCategory::BlogPost => BlogPost::read(path),
            PageCategory::Dummy => Dummy::read(path),
            PageCategory::Curriculum => CurriculumPage::read(path),
        }
    }
}

fn doc_from_path_and_locale(path: &Path, locale: Locale) -> Result<Page, DocError> {
    let mut file = root_for_locale(locale)?.to_path_buf();
    file.push(locale.as_folder_str());
    file.push(path);
    file.push("index.md");
    Doc::read(file)
}

pub fn url_path_to_page(url_path: &str) -> Result<Page, DocError> {
    url_path_to_page_with_other_locale_and_fallback(url_path, None)
}

pub fn url_path_to_page_with_other_locale_and_fallback(
    url_path: &str,
    locale: Option<Locale>,
) -> Result<Page, DocError> {
    if let Some(dummy) = Dummy::from_url(url_path) {
        return Ok(dummy);
    }
    let (path, locale_from_url, typ) = url_path_to_path_buf(url_path)?;
    let locale = locale.unwrap_or(locale_from_url);
    match typ {
        PageCategory::Doc => {
            let doc = doc_from_path_and_locale(&path, locale);
            if doc.is_err() && locale != Default::default() {
                doc_from_path_and_locale(&path, Default::default())
            } else {
                doc
            }
        }
        PageCategory::BlogPost => blog_from_url(url_path).ok_or(DocError::PageNotFound(
            url_path.to_string(),
            PageCategory::BlogPost,
        )),
        PageCategory::Curriculum => curriculum_from_url(&url_path.to_ascii_lowercase()).ok_or(
            DocError::PageNotFound(url_path.to_string(), PageCategory::Curriculum),
        ),
        _ => unreachable!(),
    }
}

#[enum_dispatch(Page)]
pub trait PageLike {
    fn url(&self) -> &str;
    fn slug(&self) -> &str;
    fn title(&self) -> &str;
    fn short_title(&self) -> Option<&str>;
    fn locale(&self) -> Locale;
    fn content(&self) -> &str;
    fn rari_env(&self) -> Option<RariEnv<'_>>;
    fn render(&self) -> Result<String, DocError>;
    fn title_suffix(&self) -> Option<&str>;
    fn page_type(&self) -> PageType;
    fn status(&self) -> &[FeatureStatus];
    fn full_path(&self) -> &Path;
    fn path(&self) -> &Path;
    fn base_slug(&self) -> &str;
    fn trailing_slash(&self) -> bool;
}

impl<T: PageLike> PageLike for Arc<T> {
    fn url(&self) -> &str {
        (**self).url()
    }

    fn slug(&self) -> &str {
        (**self).slug()
    }

    fn title(&self) -> &str {
        (**self).title()
    }

    fn short_title(&self) -> Option<&str> {
        (**self).short_title()
    }

    fn locale(&self) -> Locale {
        (**self).locale()
    }

    fn content(&self) -> &str {
        (**self).content()
    }

    fn rari_env(&self) -> Option<RariEnv<'_>> {
        (**self).rari_env()
    }

    fn render(&self) -> Result<String, DocError> {
        (**self).render()
    }

    fn title_suffix(&self) -> Option<&str> {
        (**self).title_suffix()
    }

    fn page_type(&self) -> PageType {
        (**self).page_type()
    }

    fn status(&self) -> &[FeatureStatus] {
        (**self).status()
    }

    fn full_path(&self) -> &Path {
        (**self).full_path()
    }

    fn path(&self) -> &Path {
        (**self).path()
    }

    fn base_slug(&self) -> &str {
        (**self).base_slug()
    }

    fn trailing_slash(&self) -> bool {
        (**self).trailing_slash()
    }
}

pub trait PageReader {
    fn read(path: impl Into<PathBuf>) -> Result<Page, DocError>;
}
