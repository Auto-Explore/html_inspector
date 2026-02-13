# html/semantics/document-metadata/the-link-element/link-load-fired-before-scripting-unblocked.html

Counts:
- errors: 2
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/document-metadata/the-link-element/link-load-fired-before-scripting-unblocked.html",
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
var saw_link_onload = false;
var t = async_test("Check if the stylesheet's load event is fired before the " +
                   "pending parsing-blocking script is unblocked");
</script>
<link href="style.css?pipe=trickle(d3)" rel="stylesheet" id="style_test"
      onload="t.step(function() { saw_link_onload = true; })"
      onerror="t.unreached_func('Sheet should load OK')">
<script>
  t.step(function() {
    assert_true(saw_link_onload, "The pending parsing-blocking script should " +
                                 "only run after the last element that " +
                                 "contributes a script-blocking style " +
                                 "sheet's load event is fired.");
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
        "byte_end": 1042,
        "byte_start": 1035,
        "col": 1,
        "line": 23
      }
    },
    {
      "category": "Html",
      "code": "html.parser.stray_end_tag",
      "message": "Stray end tag “html”.",
      "severity": "Error",
      "span": {
        "byte_end": 1050,
        "byte_start": 1043,
        "col": 1,
        "line": 24
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
  "source_name": "html/semantics/document-metadata/the-link-element/link-load-fired-before-scripting-unblocked.html"
}
```
