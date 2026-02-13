# html/semantics/embedded-content/the-iframe-element/iframe-display-none-with-object.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-iframe-element/iframe-display-none-with-object.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset=utf-8>
<title>Test that iframe with object triggers load event in owner document</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<body>
<script>
  async_test(t => {
    window.onload = t.step_func_done();
    const obj = document.createElement("iframe");
    obj.style.display = "none";
    obj.src = "support/iframe-with-object.html";
    document.body.appendChild(obj);
  }, "Load event triggered on window");
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
  "source_name": "html/semantics/embedded-content/the-iframe-element/iframe-display-none-with-object.html"
}
```
