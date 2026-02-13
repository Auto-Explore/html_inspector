# html/semantics/scripting-1/the-script-element/css-module/script-element-css-src.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/scripting-1/the-script-element/css-module/script-element-css-src.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>&lt;script&gt; with CSS src</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script>
  window.log = [];

  const test_load = async_test(
      "Test that <script> doesn't load when the src is CSS.");
  window.addEventListener("load", test_load.step_func_done(ev => {
    assert_array_equals(log, ["error"]);
  }));
</script>
<script type="module" src="./resources/basic.css" onload="t.unreached_func('CSS src should fail to load')" onerror="log.push('error')"></script>
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
  "source_name": "html/semantics/scripting-1/the-script-element/css-module/script-element-css-src.html"
}
```
