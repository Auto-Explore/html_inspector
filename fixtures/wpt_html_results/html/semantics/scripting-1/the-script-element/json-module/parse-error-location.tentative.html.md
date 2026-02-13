# html/semantics/scripting-1/the-script-element/json-module/parse-error-location.tentative.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/scripting-1/the-script-element/json-module/parse-error-location.tentative.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset=utf-8>
<title>JSON modules: parse error file location</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id=log></div>
<script>
setup({
  allow_uncaught_exception: true,
});
async_test(t => {
  window.addEventListener("error", t.step_func_done(e => {
    // The specific values of these properties are implementation-defined
    // per https://html.spec.whatwg.org/#report-an-exception
    // and https://html.spec.whatwg.org/#extract-error,
    // but it's preferable if implementations provide the
    // correct file location.
    assert_equals(e.filename, new URL("parse-error.json", location).href);
  }));
});
</script>
<script type="module">
import v from "./parse-error.json" with { type: "json" };
</script>
```

```json
{
  "messages": [
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/semantics/scripting-1/the-script-element/json-module/parse-error-location.tentative.html"
}
```
