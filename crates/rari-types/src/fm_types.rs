use serde::{Deserialize, Serialize};
use strum::EnumString;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, EnumString)]
#[serde(rename_all = "kebab-case")]
#[strum(serialize_all = "kebab-case")]
pub enum FeatureStatus {
    Experimental,
    NonStandard,
    Deprecated,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, EnumString)]
#[serde(rename_all = "kebab-case")]
#[strum(serialize_all = "kebab-case")]
pub enum PageType {
    #[default]
    None,
    Guide,
    LandingPage,
    HowTo,
    Tutorial,
    TutorialChapter,
    LearnTopic,
    LearnModule,
    LearnModuleChapter,
    LearnModuleAssessment,
    LearnFaq,
    AriaRole,
    AriaAttribute,
    CssAtRule,
    CssAtRuleDescriptor,
    CssCombinator,
    CssFunction,
    CssKeyword,
    CssMediaFeature,
    CssModule,
    CssProperty,
    CssPseudoClass,
    CssPseudoElement,
    CssSelector,
    CssShorthandProperty,
    CssType,
    GlossaryDefinition,
    GlossaryDisambiguation,
    HtmlAttribute,
    HtmlAttributeValue,
    HtmlElement,
    HttpCspDirective,
    HttpCorsError,
    HttpPermissionsPolicyDirective,
    HttpHeader,
    HttpMethod,
    HttpStatusCode,
    JavascriptClass,
    JavascriptConstructor,
    JavascriptError,
    JavascriptFunction,
    JavascriptGlobalProperty,
    JavascriptInstanceAccessorProperty,
    JavascriptInstanceDataProperty,
    JavascriptInstanceMethod,
    JavascriptLanguageFeature,
    JavascriptNamespace,
    JavascriptOperator,
    JavascriptStatement,
    JavascriptStaticAccessorProperty,
    JavascriptStaticDataProperty,
    JavascriptStaticMethod,
    MathmlAttribute,
    MathmlElement,
    SvgAttribute,
    SvgElement,
    WebApiOverview,
    WebApiGlobalFunction,
    WebApiGlobalProperty,
    WebApiInterface,
    WebApiConstructor,
    WebApiInstanceMethod,
    WebApiInstanceProperty,
    WebApiStaticMethod,
    WebApiStaticProperty,
    WebApiEvent,
    WebglExtension,
    WebglExtensionMethod,
    WebassemblyFunction,
    WebassemblyConstructor,
    WebassemblyInterface,
    WebassemblyInstanceProperty,
    WebassemblyInstanceMethod,
    WebassemblyStaticMethod,
    WebassemblyInstruction,
    WebdriverCommand,
    WebdriverCapability,
    WebdriverError,
    WebextensionApi,
    WebextensionApiEvent,
    WebextensionApiFunction,
    WebextensionApiProperty,
    WebextensionApiType,
    WebextensionManifestKey,
    WebManifestMember,
    XpathFunction,
    XsltElement,
    XsltAxis,
    XsltFunction,
    FirefoxReleaseNotes,

    // Synthetic
    BlogPost,
    Curriculum,

    #[serde(other)]
    SPA,
}
