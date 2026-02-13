# html/interaction/focus/focus-file-input.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/interaction/focus/focus-file-input.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>HTML Test: file input can be programatically focused before layout</title>
<link rel="help" href="https://html.spec.whatwg.org/multipage/#focus">
<link rel="help" href="https://bugzilla.mozilla.org/show_bug.cgi?id=505355">
<link rel="author" title="Mozilla" href="https://mozilla.org/">
<link rel="author" title="Emilio Cobos Álvarez" href="mailto:emilio@crisal.io">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<input type="file">
<script>
test(function() {
  let input = document.querySelector("input");
  assert_not_equals(document.activeElement, input);
  input.focus();
  assert_equals(document.activeElement, input);
});
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
  "source_name": "html/interaction/focus/focus-file-input.html"
}
```
