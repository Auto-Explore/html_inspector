# html/dom/render-blocking/parser-blocking-script.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/dom/render-blocking/parser-blocking-script.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Parser-blocking script elements are implicitly render-blocking</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="support/test-render-blocking.js"></script>

<script>
// Add some renderable content before parser inserts body
document.documentElement.appendChild(document.createTextNode('text'));

// Test must be setup before the parser-blocking script
test_render_blocking(
    () => assert_equals(window.dummy, 1),
    'Parser-blocking script is evaluated');
</script>

<script src="support/dummy-1.js?pipe=trickle(d1)"></script>

<div>Some more text</div>
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
  "source_name": "html/dom/render-blocking/parser-blocking-script.html"
}
```
