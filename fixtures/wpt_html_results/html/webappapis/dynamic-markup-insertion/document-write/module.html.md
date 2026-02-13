# html/webappapis/dynamic-markup-insertion/document-write/module.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/webappapis/dynamic-markup-insertion/document-write/module.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset=utf-8>
<title>document.write in a module</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id="log"></div>
<script>
async_test(t => {
  const iframe = document.createElement("iframe");

  iframe.onerror = t.unreached_func("Error loading iframe");
  document.addEventListener("documentWriteDone", t.step_func(() => {
    assert_equals(iframe.contentDocument.body.textContent, "Initial body contents\n");
  }));
  iframe.onload = t.step_func_done(() => {
    assert_equals(iframe.contentDocument.body.textContent, "Initial body contents\n");
  });

  iframe.src = "module-iframe.html";
  document.body.appendChild(iframe);
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
  "source_name": "html/webappapis/dynamic-markup-insertion/document-write/module.html"
}
```
