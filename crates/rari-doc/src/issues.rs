use std::collections::{BTreeMap, HashMap};
use std::fmt;
use std::sync::atomic::AtomicU64;
use std::sync::{Arc, Mutex};

use serde::Serialize;
use tracing::field::{Field, Visit};
use tracing::span::{Attributes, Id};
use tracing::{Event, Subscriber};
use tracing_subscriber::registry::LookupSpan;
use tracing_subscriber::Layer;

static ISSUE_COUNTER: AtomicU64 = AtomicU64::new(1);

pub(crate) fn get_issue_couter() -> u64 {
    ISSUE_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
}

#[derive(Debug, Default, Clone)]
pub struct Issue {
    pub req: u64,
    pub ic: u64,
    pub fields: Vec<(&'static str, String)>,
    pub spans: Vec<(&'static str, String)>,
}

#[derive(Debug, Default)]
pub struct IssueEntries {
    req: u64,
    ic: u64,
    entries: Vec<(&'static str, String)>,
}

#[derive(Clone, Debug, Serialize)]
pub struct Issues<'a> {
    pub templ: BTreeMap<&'a str, Vec<TemplIssue<'a>>>,
    pub other: BTreeMap<&'a str, Vec<TemplIssue<'a>>>,
    pub no_pos: BTreeMap<&'a str, Vec<TemplIssue<'a>>>,
}

#[derive(Clone, Debug, Serialize)]
pub struct TemplIssue<'a> {
    pub req: u64,
    pub ic: u64,
    pub source: &'a str,
    pub file: &'a str,
    pub slug: &'a str,
    pub locale: &'a str,
    pub line: &'a str,
    pub col: &'a str,
    pub tail: Vec<(&'static str, &'a str)>,
}

static UNKNOWN: &str = "unknown";
static DEFAULT_TEMPL_ISSUE: TemplIssue<'static> = TemplIssue {
    req: 0,
    ic: 0,
    source: UNKNOWN,
    file: UNKNOWN,
    slug: UNKNOWN,
    locale: UNKNOWN,
    line: UNKNOWN,
    col: UNKNOWN,
    tail: vec![],
};

impl<'a> From<&'a Issue> for TemplIssue<'a> {
    fn from(value: &'a Issue) -> Self {
        let mut tissue = DEFAULT_TEMPL_ISSUE.clone();
        for (key, value) in value.spans.iter().chain(value.fields.iter()) {
            match *key {
                "file" => {
                    tissue.file = value.as_str();
                }
                "slug" => {
                    tissue.slug = value.as_str();
                }
                "locale" => {
                    tissue.locale = value.as_str();
                }
                "line" => tissue.line = value.as_str(),
                "col" => tissue.col = value.as_str(),
                "source" => {
                    tissue.source = value.as_str();
                }
                "message" => {}
                _ => tissue.tail.push((key, value.as_str())),
            }
        }
        tissue
    }
}

pub fn issues_by(issues: &[Issue]) -> Issues {
    let mut templ: BTreeMap<&str, Vec<TemplIssue>> = BTreeMap::new();
    let mut other: BTreeMap<&str, Vec<TemplIssue>> = BTreeMap::new();
    let mut no_pos: BTreeMap<&str, Vec<TemplIssue>> = BTreeMap::new();
    for issue in issues.iter().map(TemplIssue::from) {
        if let Some(templ_name) =
            issue
                .tail
                .iter()
                .find_map(|(key, value)| if *key == "templ" { Some(value) } else { None })
        {
            templ.entry(templ_name).or_default().push(issue);
        } else if issue.line != UNKNOWN {
            other.entry(issue.source).or_default().push(issue)
        } else {
            no_pos.entry(issue.source).or_default().push(issue);
        }
    }
    Issues {
        templ,
        other,
        no_pos,
    }
}

#[derive(Clone, Default)]
pub struct InMemoryLayer {
    events: Arc<Mutex<Vec<Issue>>>,
}

impl InMemoryLayer {
    pub fn get_events(&self) -> Arc<Mutex<Vec<Issue>>> {
        Arc::clone(&self.events)
    }
}

impl Visit for IssueEntries {
    fn record_debug(&mut self, field: &Field, value: &dyn fmt::Debug) {
        self.entries.push((field.name(), format!("{value:?}")));
    }
    fn record_str(&mut self, field: &Field, value: &str) {
        self.entries.push((field.name(), value.to_string()));
    }
    fn record_u64(&mut self, field: &Field, value: u64) {
        if field.name() == "req" {
            self.req = value;
        } else if field.name() == "ic" {
            self.ic = value;
        }
    }
}
impl Visit for Issue {
    fn record_debug(&mut self, field: &Field, value: &dyn fmt::Debug) {
        self.fields.push((field.name(), format!("{value:?}")));
    }
    fn record_str(&mut self, field: &Field, value: &str) {
        self.fields.push((field.name(), value.to_string()));
    }
    fn record_u64(&mut self, field: &Field, value: u64) {
        if field.name() == "req" {
            self.req = value;
        } else if field.name() == "ic" {
            self.ic = value;
        }
    }
}
impl<S> Layer<S> for InMemoryLayer
where
    S: Subscriber + for<'a> LookupSpan<'a>,
{
    fn on_new_span(
        &self,
        attrs: &Attributes<'_>,
        id: &Id,
        ctx: tracing_subscriber::layer::Context<S>,
    ) {
        let span = ctx.span(id).expect("Span not found, this is a bug");
        let mut extensions = span.extensions_mut();

        if extensions.get_mut::<IssueEntries>().is_none() {
            let mut fields = IssueEntries::default();
            attrs.values().record(&mut fields);
            extensions.insert(fields);
        }
    }
    fn on_event(&self, event: &Event, ctx: tracing_subscriber::layer::Context<S>) {
        let mut issue = Issue {
            req: 0,
            ic: 0,
            fields: vec![],
            spans: vec![],
        };
        let span = ctx.event_span(event);
        let scope = span.into_iter().flat_map(|span| span.scope());
        for span in scope {
            let ext = span.extensions();
            if let Some(entries) = ext.get::<IssueEntries>() {
                if entries.req != 0 {
                    issue.req = entries.req;
                }
                if entries.ic != 0 {
                    issue.ic = entries.ic;
                } else {
                    issue.ic = get_issue_couter();
                }
                issue.spans.extend(entries.entries.iter().rev().cloned());
            }
        }

        event.record(&mut issue);
        let mut events = self.events.lock().unwrap();
        events.push(issue);
    }
}

#[derive(Serialize, Debug, Default, Clone)]
#[serde(untagged)]
pub enum Additional {
    BrokenLink {
        href: String,
    },
    #[default]
    None,
}

#[derive(Serialize, Debug, Default, Clone)]
pub struct DisplayIssue {
    pub id: u64,
    pub explanation: Option<String>,
    pub suggestion: Option<String>,
    pub fixable: Option<bool>,
    pub fixed: bool,
    pub name: String,
    pub line: Option<usize>,
    pub col: Option<usize>,
    #[serde(flatten)]
    pub additional: Additional,
}

pub type DisplayIssues = BTreeMap<&'static str, Vec<DisplayIssue>>;

impl From<Issue> for DisplayIssue {
    fn from(value: Issue) -> Self {
        let mut di = DisplayIssue {
            id: value.ic,
            ..Default::default()
        };
        let mut additional = HashMap::new();
        for (key, value) in value.spans.into_iter().chain(value.fields.into_iter()) {
            match key {
                "line" => di.line = value.parse().ok(),
                "col" => di.col = value.parse().ok(),
                "source" => {
                    di.name = value;
                }
                "message" => di.explanation = Some(value),
                "redirect" => di.suggestion = Some(value),

                _ => {
                    additional.insert(key, value);
                }
            }
        }
        let additional = match di.name.as_str() {
            "redirected-link" => {
                di.fixed = false;
                di.fixable = Some(true);
                Additional::BrokenLink {
                    href: additional.remove("url").unwrap_or_default(),
                }
            }
            _ => Additional::None,
        };
        di.additional = additional;
        di
    }
}

pub fn to_display_issues(issues: Vec<Issue>) -> DisplayIssues {
    let mut map = BTreeMap::new();
    for issue in issues {
        let di = DisplayIssue::from(issue);
        match &di.additional {
            Additional::BrokenLink { .. } => {
                let entry: &mut Vec<_> = map.entry("broken_links").or_default();
                entry.push(di);
            }
            Additional::None => {
                let entry: &mut Vec<_> = map.entry("unknown").or_default();
                entry.push(di);
            }
        }
    }
    map
}
