# html/semantics/document-metadata/the-link-element/link-error-fired-before-scripting-unblocked.html

Counts:
- errors: 2
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/document-metadata/the-link-element/link-error-fired-before-scripting-unblocked.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel="author" title="Dom Farolino" href="mailto:dom@chromium.org">
<link rel="help" href="https://html.spec.whatwg.org/multipage/links.html#link-type-stylesheet">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script>
var saw_link_onerror = false;
var t = async_test("Check if the stylesheet's error event is fired before the " +
                   "pending parsing-blocking script is unblocked");
</script>
<link href="nonexistent.css" rel="stylesheet" id="style_test"
      onload="t.unreached_func('Sheet should fail to load')"
      onerror="t.step(function() { saw_link_onerror = true; })">
<script>
  t.step(function() {
    assert_true(saw_link_onerror, "The pending parsing-blocking script should " +
                                  "only run after the last element that " +
                                  "contributes a script-blocking style " +
                                  "sheet's error event is fired if the sheet " +
                                  "fails to load.");
  });
  t.done();
</script>
</head>
</html>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.parser.stray_end_tag",
      "message": "Stray end tag “head”.",
      "severity": "Error",
      "span": {
        "byte_end": 1110,
        "byte_start": 1103,
        "col": 1,
        "line": 24
      }
    },
    {
      "category": "Html",
      "code": "html.parser.stray_end_tag",
      "message": "Stray end tag “html”.",
      "severity": "Error",
      "span": {
        "byte_end": 1118,
        "byte_start": 1111,
        "col": 1,
        "line": 25
      }
    },
    {
      "category": "Html",
      "code": "html.head.title.missing",
      "message": "Element “head” is missing a required instance of child element “title”.",
      "severity": "Warning",
      "span": null
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/semantics/document-metadata/the-link-element/link-error-fired-before-scripting-unblocked.html"
}
```
