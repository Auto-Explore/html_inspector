# html/semantics/embedded-content/the-iframe-element/iframe_sandbox_navigate_itself.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-iframe-element/iframe_sandbox_navigate_itself.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset="utf-8">
<title>Check that sandboxed iframe can navigate itself</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script>
  var t = async_test();
  onmessage = t.step_func_done(function(e) {
    assert_equals(e.data, "can navigate", "Should have the right message");
  });
</script>
<iframe sandbox="allow-scripts" src="support/iframe-trying-to-navigate-itself.html"></iframe>
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
  "source_name": "html/semantics/embedded-content/the-iframe-element/iframe_sandbox_navigate_itself.html"
}
```
