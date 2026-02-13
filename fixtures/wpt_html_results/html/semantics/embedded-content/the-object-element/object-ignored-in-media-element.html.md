# html/semantics/embedded-content/the-object-element/object-ignored-in-media-element.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-object-element/object-ignored-in-media-element.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset="utf-8">
<title>HTML Test: The embed element represents a document</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<meta name="assert" content="Check if the object element is ignored when used inside a media element">
<script type="application/javascript">
  var nestingTest = async_test("Test <object> being ignored inside media element");
  onload = nestingTest.step_func_done(function() {
    assert_true(true, "We got to a load event without loading things we should not load");
  });
</script>
<body>
  <video>
    <object type="text/html" data="../resources/should-not-load.html"
            test-description="<object> in <video>"></object>
  </video>
  <audio>
    <object type="text/html" data="../resources/should-not-load.html"
            test-description="<object> in <audio>"></object>
  </audio>
</body>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.script.type.unnecessary",
      "message": "The “type” attribute is unnecessary for JavaScript resources.",
      "severity": "Warning",
      "span": {
        "byte_end": 352,
        "byte_start": 314,
        "col": 1,
        "line": 7
      }
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/semantics/embedded-content/the-object-element/object-ignored-in-media-element.html"
}
```
