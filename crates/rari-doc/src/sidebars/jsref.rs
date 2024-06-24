/*
// Data for related pages
const groupData = {
  Error: [
    "Error",
    ...Object.keys(inheritanceData).filter((key) =>
      inheritanceData[key].includes("Error")
    ),
  ],
  Intl: [
    "Intl",
    ...(await page
      .subpagesExpand(
        "/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl",
      )
      .filter((page) => page.pageType === "javascript-class")
      .map((page) => slugToObjName(page.slug))),
  ],
  Segments: [
    "Intl/Segmenter/segment/Segments",
    "Intl.Segmenter",
  ],
  TypedArray: [
    "TypedArray",
    ...Object.keys(inheritanceData).filter((key) =>
      inheritanceData[key].includes("TypedArray")
    ),
  ],
  Proxy: ["Proxy", "Proxy/handler"],
};
*/

use std::borrow::Cow;

pub fn sidebar(slug: &str) {
    let main_object = slug_to_object_name(slug);
    let mut inheritance = vec![main_object.as_ref()];
    if let Some(data) = inheritance_data(&main_object) {
        inheritance.push(data);
    }
    if !matches!(
        main_object.as_ref(),
        "Proxy" | "Atomics" | "Math" | "Intl" | "JSON" | "Reflect",
    ) {
        // %Base% is the default inheritance when the class has no extends clause:
        // instances inherit from Object.prototype, and class inherits from Function.prototype
        inheritance.push("%Base%")
    }
}

fn inheritance_data(obj: &str) -> Option<&str> {
    match obj {
        "AsyncGenerator" => Some("AsyncIterator"),
        "AsyncFunction" | "AsyncGeneratorFunction" => Some("Function"),
        "GeneratorFunction" => Some("Function"),
        "Generator" => Some("Iterator"),
        // TypedArray
        "BigInt64Array" | "BigUint64Array" | "Float32Array" | "Float64Array" | "Int8Array"
        | "Int16Array" | "Int32Array" | "Uint8Array" | "Uint8ClampedArray" | "Uint16Array"
        | "Uint32Array" => Some("TypedArray"),
        // Error
        "AggregateError" | "EvalError" | "InternalError" | "RangeError" | "ReferenceError"
        | "SyntaxError" | "TypeError" | "URIError" => Some("Error"),
        _ => None,
    }
}

fn slug_to_object_name(slug: &str) -> Cow<'_, str> {
    let sub_path = slug
        .strip_prefix("Web/JavaScript/Reference/Global_Objects/")
        .unwrap_or_default();
    if sub_path.starts_with("Intl/Segmenter/segment/Segments") {
        return "Intl/Segmenter/segment/Segments".into();
    }
    if sub_path.starts_with("Proxy/Proxy") {
        return "Proxy/handler".into();
    }
    if let Some(intl) = sub_path.strip_prefix("Intl/") {
        if intl.chars().all(|c| c.is_ascii_alphabetic()) {
            return "Intl".into();
        }
        return Cow::Owned(
            sub_path[..intl.find('/').map(|i| i + 5).unwrap_or(sub_path.len())].replace('/', "."),
        );
    }

    sub_path[..sub_path.find('/').unwrap_or(sub_path.len())].into()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_slug_to_object_name() {
        assert_eq!(
            slug_to_object_name("Web/JavaScript/Reference/Global_Objects/Intl/supportedValuesOf"),
            "Intl"
        );
        assert_eq!(
            slug_to_object_name(
                "Web/JavaScript/Reference/Global_Objects/Intl/DateTimeFormat/format"
            ),
            "Intl.DateTimeFormat"
        );
        assert_eq!(
            slug_to_object_name(
                "Web/JavaScript/Reference/Global_Objects/ArrayBuffer/maxByteLength"
            ),
            "ArrayBuffer"
        );
        assert_eq!(
            slug_to_object_name(
                "Web/JavaScript/Reference/Global_Objects/ArrayBuffer/maxByteLength"
            ),
            "ArrayBuffer"
        );
        assert_eq!(
            slug_to_object_name("Web/JavaScript/Reference/Global_Objects/Array"),
            "Array"
        );
    }
}
