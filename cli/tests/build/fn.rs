use super::*;

fn write_snippet(dir: &Path, body: &str) {
    fs::create_dir_all(dir).unwrap();
    fs::write(dir.join("inline0.js"), body).unwrap();
}

#[tokio::test]
async fn minifies_uncompressed_snippet() {
    let tmp = tempfile::tempdir().unwrap();
    let pkg = tmp.path().join("pkg");
    let crate_dir = pkg.join("snippets").join("euv-core-abc123");
    let original = include_str!("minify_inline_js_snippet_input.txt");
    write_snippet(&crate_dir, original);
    euv_cli::minify_inline_js_snippets(&pkg).await.unwrap();
    let minified = fs::read_to_string(crate_dir.join("inline0.js")).unwrap();
    assert!(
        minified.len() < original.len() / 2,
        "minified({}) should be < 50% of original({})",
        minified.len(),
        original.len()
    );
    assert!(minified.contains("euv_event_collect_id_chain"));
    assert!(!minified.contains("//"));
    assert!(!minified.contains("\n    "));
}

#[tokio::test]
async fn skips_missing_snippets_dir() {
    let tmp = tempfile::tempdir().unwrap();
    let pkg = tmp.path().join("pkg");
    euv_cli::minify_inline_js_snippets(&pkg).await.unwrap();
}

#[tokio::test]
async fn leaves_non_inline_js_files_alone() {
    let tmp = tempfile::tempdir().unwrap();
    let pkg = tmp.path().join("pkg");
    let crate_dir = pkg.join("snippets").join("euv-core-abc123");
    fs::create_dir_all(&crate_dir).unwrap();
    let unrelated = "// not an inline snippet\nconst x = 1;\n";
    fs::write(crate_dir.join("helper.js"), unrelated).unwrap();
    euv_cli::minify_inline_js_snippets(&pkg).await.unwrap();
    let untouched = fs::read_to_string(crate_dir.join("helper.js")).unwrap();
    assert_eq!(untouched, unrelated);
}

const HTML_INPUT: &str = "<!doctype html>\n<html lang=\"en\">\n  <head>\n    <meta charset=\"utf-8\" />\n    <!-- comment -->\n    <title>Euv</title>\n  </head>\n  <body>\n    <div id=\"app\">    </div>\n    <script>  const x = 1;   const y = 2;  </script>\n    <style>body  {   margin: 0;  }</style>\n  </body>\n</html>\n";
const HTML_EXPECTED: &str = "<!doctype html><html lang=\"en\"><head><meta charset=\"utf-8\" /><title>Euv</title></head><body><div id=\"app\"></div><script>  const x = 1;   const y = 2;  </script><style>body  {   margin: 0;  }</style></body></html>";

#[test]
fn minifies_html_template() {
    let minified = euv_cli::minify_html_template(HTML_INPUT);
    assert_eq!(minified, HTML_EXPECTED);
}

#[test]
fn html_minify_strips_comments_and_indentation() {
    let input = "<div>\n  <!-- keep nothing -->\n  <span>   hello   world   </span>\n</div>\n";
    let minified = euv_cli::minify_html_template(input);
    assert!(!minified.contains("<!--"), "got: {minified:?}");
    assert!(!minified.contains("  "), "got: {minified:?}");
    assert!(!minified.contains("\n"), "got: {minified:?}");
    assert!(minified.contains("hello world"), "got: {minified:?}");
}

#[test]
fn html_minify_preserves_script_and_style_contents() {
    let input = "<script>  if (a)  {  return  b  ;  }  </script>\n";
    let minified = euv_cli::minify_html_template(input);
    assert!(minified.contains("  if (a)  {  return  b  ;  }  "));
}

#[test]
fn html_minify_shrinks_realistic_template() {
    let input = "<!doctype html>\n<html lang=\"en\">\n  <head>\n    <meta charset=\"utf-8\" />\n    <title>Euv</title>\n  </head>\n  <body>\n    <div id=\"app\"></div>\n    <script type=\"module\">init();</script>\n  </body>\n</html>\n";
    let minified = euv_cli::minify_html_template(input);
    assert!(minified.len() < input.len());
    assert!(!minified.contains("\n"));
    assert!(!minified.contains("  "));
    assert!(minified.contains("__NOTHING__") || minified.contains("init"));
}
