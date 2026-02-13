# html/browsers/the-window-object/window-open-invalid-url.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/the-window-object/window-open-invalid-url.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset=utf-8>
<title>window.open() with an invalid URL</title>
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>
<script>
test(() => {
  assert_throws_dom("SyntaxError", () => window.open("https://example.com\u0000mozilla.org"));
}, "Window.open should throw SyntaxError when an invalid url is passed as the first parameter");
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
  "source_name": "html/browsers/the-window-object/window-open-invalid-url.html"
}
```
