# html/semantics/document-metadata/the-link-element/link-load-event.html

Counts:
- errors: 2
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/document-metadata/the-link-element/link-load-event.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel="author" title="Josh Matthews" href="mailto:josh@joshmatthews.net">
<link rel="help" href="https://html.spec.whatwg.org/multipage/#the-link-element">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script>
var saw_link_onload = false;
var t = async_test("Check if the stylesheet's load event blocks the document load event");
window.addEventListener('load', t.step_func_done(function() {
  assert_true(saw_link_onload);
}));
</script>
<link href="style.css?pipe=trickle(d3)" rel="stylesheet" id="style_test"
      onload="t.step(function() { saw_link_onload = true; })"
      onerror="t.unreached_func('Sheet should load OK')">
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
        "byte_end": 720,
        "byte_start": 713,
        "col": 1,
        "line": 16
      }
    },
    {
      "category": "Html",
      "code": "html.parser.stray_end_tag",
      "message": "Stray end tag “html”.",
      "severity": "Error",
      "span": {
        "byte_end": 728,
        "byte_start": 721,
        "col": 1,
        "line": 17
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
  "source_name": "html/semantics/document-metadata/the-link-element/link-load-event.html"
}
```
