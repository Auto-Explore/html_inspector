# html/webappapis/dynamic-markup-insertion/document-write/module-static-import.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/webappapis/dynamic-markup-insertion/document-write/module-static-import.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>document.write in an imported module</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id="log"></div>
<script>
async_test(t => {
  const iframe = document.createElement("iframe");

  iframe.onerror = t.unreached_func("Error loading iframe");

  let testEndWasCalled = false;
  document.addEventListener("documentWriteDone", t.step_func(() => {
    testEndWasCalled = true;
    assert_equals(iframe.contentDocument.body.textContent, "Initial body contents\n");
  }));
  iframe.onload = t.step_func_done(() => {
    assert_true(testEndWasCalled, "onload must be called");
    assert_equals(iframe.contentDocument.body.textContent, "Initial body contents\n");
  });

  iframe.src = "module-static-import-iframe.html";
  document.body.appendChild(iframe);
});
</script>
ß
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
  "source_name": "html/webappapis/dynamic-markup-insertion/document-write/module-static-import.html"
}
```
