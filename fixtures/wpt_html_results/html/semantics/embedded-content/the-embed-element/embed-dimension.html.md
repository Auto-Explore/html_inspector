# html/semantics/embedded-content/the-embed-element/embed-dimension.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-embed-element/embed-dimension.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>HTML Test: dimension</title>
<link rel="author" title="Intel" href="http://www.intel.com">
<link rel="help" href="https://html.spec.whatwg.org/multipage/#the-embed-element">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id="log"></div>
<embed src="/images/blue.png" height="100" width="100" id="test">
<script>
  test(function () {
    var height = getComputedStyle(document.getElementById("test"))["height"];
    assert_equals(height, "100px", "The height of the embed element should be 100px.");
  }, "Check the actual length of the embed element's height");

  test(function () {
    var width = getComputedStyle(document.getElementById("test"))["width"];
    assert_equals(width, "100px", "The width of the embed element should be 100px.");
  }, "Check the actual length of the embed element's width");
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
  "source_name": "html/semantics/embedded-content/the-embed-element/embed-dimension.html"
}
```
