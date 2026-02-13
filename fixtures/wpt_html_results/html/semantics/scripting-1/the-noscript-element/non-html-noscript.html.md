# html/semantics/scripting-1/the-noscript-element/non-html-noscript.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/scripting-1/the-noscript-element/non-html-noscript.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>noscript rules don't apply to non-html elements</title>
<link rel="author" title="Emilio Cobos Álvarez" href="mailto:emilio@crisal.io">
<link rel="help" href="https://html.spec.whatwg.org/multipage/#the-noscript-element">
<link rel="help" href="https://bugzilla.mozilla.org/show_bug.cgi?id=1470150">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<body>
<script>
test(function() {
  let non_html_noscript = document.createElementNS("http://www.w3.org/2000/svg", "noscript");
  document.body.appendChild(non_html_noscript);
  assert_not_equals(getComputedStyle(non_html_noscript).display, "none");
}, "Non-HTML <noscript> element shouldn't be undisplayed by a UA rule");
</script>
</body>
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
  "source_name": "html/semantics/scripting-1/the-noscript-element/non-html-noscript.html"
}
```
