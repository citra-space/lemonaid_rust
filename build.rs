use std::env;
use std::fs;
use std::path::PathBuf;

use serde_json::Value;

fn main() {
    let spec_path = "openapi/citra.json";
    println!("cargo:rerun-if-changed={spec_path}");
    println!("cargo:rerun-if-changed=build.rs");

    let raw = fs::read_to_string(spec_path)
        .unwrap_or_else(|e| panic!("read {spec_path}: {e}"));
    let mut value: Value = serde_json::from_str(&raw)
        .unwrap_or_else(|e| panic!("parse {spec_path} as JSON: {e}"));

    downconvert_3_1_to_3_0(&mut value);

    // Strip all `default` values. typify validates numeric defaults against
    // their declared bounds, and the spec contains at least one combination
    // that fails its own constraints once references are resolved. We lose
    // the convenience of server-side defaults flowing through the builder
    // API, which is acceptable — callers construct request bodies explicitly.
    strip_key(&mut value, "default");

    // progenitor cannot represent an operation whose error responses mix
    // body shapes (e.g. a 404 without a body next to a 422 with a typed
    // `HTTPValidationError` body). Drop bodyless 4xx/5xx responses; the
    // generated client falls back to `Error::UnexpectedResponse(response)`
    // for those status codes, which still surfaces the raw response.
    strip_bodyless_error_responses(&mut value);

    // typify names generated types from each schema's `title`. FastAPI
    // attaches identical titles ("Status", "Type", …) to inline property
    // schemas across unrelated parent types, which collides into a single
    // Rust enum that has the variants of the first-encountered shape only.
    // Strip `title` from inline schemas underneath property definitions so
    // typify falls back to path-based names like `<Parent><Property>`,
    // which are unique by construction. Top-level entries in
    // `components.schemas` keep their titles so their type names stay
    // stable.
    strip_inline_property_titles(&mut value);

    let normalized = serde_json::to_string(&value).expect("serialize normalized spec");
    let spec: openapiv3::OpenAPI = serde_json::from_str(&normalized)
        .unwrap_or_else(|e| panic!("parse downconverted spec as OpenAPI 3.0: {e}"));

    let mut generator = progenitor::Generator::default();
    let tokens = generator
        .generate_tokens(&spec)
        .unwrap_or_else(|e| panic!("progenitor codegen failed: {e}"));
    let ast = syn::parse2(tokens).expect("parse generated TokenStream as syn AST");
    let content = prettyplease::unparse(&ast);

    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR not set"));
    let out_file = out_dir.join("citra_api.rs");
    fs::write(&out_file, content)
        .unwrap_or_else(|e| panic!("write {}: {e}", out_file.display()));
}

/// In-place rewrite of an OpenAPI 3.1 document into something the `openapiv3`
/// crate (3.0.x) can ingest. The Citra API spec is emitted by FastAPI, which
/// uses 3.1's `anyOf: [X, {type: "null"}]` idiom for nullable fields; openapiv3
/// only understands the 3.0 `nullable: true` flag, so we collapse those.
fn downconvert_3_1_to_3_0(value: &mut Value) {
    if let Value::Object(map) = value {
        if map
            .get("openapi")
            .and_then(Value::as_str)
            .is_some_and(|v| v.starts_with("3.1"))
        {
            map.insert("openapi".into(), Value::String("3.0.3".into()));
        }

        if let Some(Value::Array(items)) = map.get("anyOf").cloned() {
            let (nulls, non_nulls): (Vec<_>, Vec<_>) = items.into_iter().partition(is_null_schema);
            if !nulls.is_empty() {
                map.remove("anyOf");
                match non_nulls.len() {
                    0 => {
                        map.insert("nullable".into(), Value::Bool(true));
                    }
                    1 => {
                        let entry = non_nulls.into_iter().next().unwrap();
                        let is_ref = entry
                            .as_object()
                            .is_some_and(|o| o.contains_key("$ref"));
                        if is_ref {
                            // OpenAPI 3.0 forbids siblings of `$ref`; wrap in
                            // allOf so we can still mark nullable.
                            map.insert("allOf".into(), Value::Array(vec![entry]));
                            map.insert("nullable".into(), Value::Bool(true));
                        } else if let Value::Object(em) = entry {
                            for (k, v) in em {
                                map.entry(k).or_insert(v);
                            }
                            map.insert("nullable".into(), Value::Bool(true));
                        }
                    }
                    _ => {
                        map.insert("anyOf".into(), Value::Array(non_nulls));
                        map.insert("nullable".into(), Value::Bool(true));
                    }
                }
            }
        }

        // 3.1's numeric `exclusiveMinimum`/`exclusiveMaximum` -> 3.0's
        // `{minimum: N, exclusiveMinimum: true}` form. FastAPI emits the
        // numeric form for `gt=`/`lt=` constraints and never emits both
        // `minimum` and a numeric `exclusiveMinimum` on the same schema, so
        // a straight lift is safe. Run this *after* the anyOf collapse so
        // bounds lifted up from a nullable wrapper get normalized too.
        for (numeric_key, paired_key) in [
            ("exclusiveMinimum", "minimum"),
            ("exclusiveMaximum", "maximum"),
        ] {
            if matches!(map.get(numeric_key), Some(Value::Number(_))) {
                let n = map.remove(numeric_key).unwrap();
                map.insert(paired_key.into(), n);
                map.insert(numeric_key.into(), Value::Bool(true));
            }
        }

        // 3.1's `const` keyword has no 3.0 equivalent — translate to a
        // single-value `enum`, which typify can still discriminate on.
        if let Some(c) = map.remove("const") {
            map.insert("enum".into(), Value::Array(vec![c]));
        }

        // OpenAPI 3.0 forbids siblings of `$ref`. The spec attaches metadata
        // (`default`, `description`, `title`, …) directly alongside `$ref`,
        // which is legal in 3.1 but not 3.0. We drop `default` outright
        // because typify cannot validate a default value against an `allOf`-
        // wrapped reference; the remaining metadata is moved into an `allOf`
        // wrapper so it can stay.
        if map.contains_key("$ref") && map.len() > 1 {
            map.remove("default");
            if map.len() > 1 {
                let ref_value = map.remove("$ref").unwrap();
                map.insert(
                    "allOf".into(),
                    Value::Array(vec![Value::Object(
                        [("$ref".to_string(), ref_value)].into_iter().collect(),
                    )]),
                );
            }
        }

        for v in map.values_mut() {
            downconvert_3_1_to_3_0(v);
        }
    } else if let Value::Array(arr) = value {
        for v in arr.iter_mut() {
            downconvert_3_1_to_3_0(v);
        }
    }
}

fn strip_inline_property_titles(value: &mut Value) {
    let Some(schemas) = value
        .pointer_mut("/components/schemas")
        .and_then(Value::as_object_mut)
    else {
        return;
    };
    for (_name, schema) in schemas.iter_mut() {
        strip_titles_inside_properties(schema);
    }
}

fn strip_titles_inside_properties(schema: &mut Value) {
    let Some(map) = schema.as_object_mut() else {
        return;
    };
    if let Some(props) = map.get_mut("properties").and_then(Value::as_object_mut) {
        for (_prop, prop_schema) in props.iter_mut() {
            strip_titles_recursively(prop_schema);
        }
    }
    for branch_key in ["allOf", "oneOf", "anyOf"] {
        if let Some(arr) = map.get_mut(branch_key).and_then(Value::as_array_mut) {
            for item in arr.iter_mut() {
                strip_titles_inside_properties(item);
            }
        }
    }
}

fn strip_titles_recursively(schema: &mut Value) {
    let Some(map) = schema.as_object_mut() else {
        return;
    };
    map.remove("title");
    if let Some(props) = map.get_mut("properties").and_then(Value::as_object_mut) {
        for (_, child) in props.iter_mut() {
            strip_titles_recursively(child);
        }
    }
    if let Some(items) = map.get_mut("items") {
        strip_titles_recursively(items);
    }
    if let Some(addl) = map.get_mut("additionalProperties") {
        if addl.is_object() {
            strip_titles_recursively(addl);
        }
    }
    for branch_key in ["allOf", "oneOf", "anyOf"] {
        if let Some(arr) = map.get_mut(branch_key).and_then(Value::as_array_mut) {
            for item in arr.iter_mut() {
                strip_titles_recursively(item);
            }
        }
    }
}

fn strip_bodyless_error_responses(value: &mut Value) {
    let Some(paths) = value.pointer_mut("/paths").and_then(Value::as_object_mut) else {
        return;
    };
    for (_path, methods) in paths.iter_mut() {
        let Some(methods) = methods.as_object_mut() else {
            continue;
        };
        for (_verb, op) in methods.iter_mut() {
            let Some(op) = op.as_object_mut() else { continue };
            let Some(responses) = op.get_mut("responses").and_then(Value::as_object_mut) else {
                continue;
            };
            responses.retain(|code, resp| {
                let is_error = code.starts_with('4') || code.starts_with('5');
                if !is_error {
                    return true;
                }
                let has_content = resp
                    .as_object()
                    .and_then(|r| r.get("content"))
                    .and_then(Value::as_object)
                    .is_some_and(|c| !c.is_empty());
                has_content
            });
        }
    }
}

fn strip_key(value: &mut Value, key: &str) {
    match value {
        Value::Object(map) => {
            map.remove(key);
            for v in map.values_mut() {
                strip_key(v, key);
            }
        }
        Value::Array(arr) => {
            for v in arr.iter_mut() {
                strip_key(v, key);
            }
        }
        _ => {}
    }
}

fn is_null_schema(v: &Value) -> bool {
    v.as_object()
        .and_then(|o| o.get("type"))
        .and_then(Value::as_str)
        == Some("null")
}
