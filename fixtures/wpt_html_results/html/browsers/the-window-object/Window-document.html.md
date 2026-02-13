# html/browsers/the-window-object/Window-document.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/the-window-object/Window-document.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset=utf-8>
<title>Window#document</title>
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>
<div id=log></div>
<script>
async_test(function() {
  var URL = "/common/blank.html";

  var iframe = document.createElement("iframe");
  document.body.appendChild(iframe);
  var initialWindow = iframe.contentWindow;
  var initialDocument = initialWindow.document;
  assert_equals(initialDocument.URL, "about:blank");
  iframe.src = URL;
  iframe.onload = this.step_func_done(function() {
    assert_equals(iframe.contentWindow, initialWindow);
    assert_equals(initialDocument.URL, "about:blank");
    var loadedDocument = initialWindow.document;
    assert_equals(loadedDocument.URL, location.href.replace(location.pathname, URL));
    assert_not_equals(initialDocument, loadedDocument);
  });
}, "Document in a browsing context");
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
  "source_name": "html/browsers/the-window-object/Window-document.html"
}
```
