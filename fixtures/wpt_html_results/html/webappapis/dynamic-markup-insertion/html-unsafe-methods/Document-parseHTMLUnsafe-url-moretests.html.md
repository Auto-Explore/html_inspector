# html/webappapis/dynamic-markup-insertion/html-unsafe-methods/Document-parseHTMLUnsafe-url-moretests.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/webappapis/dynamic-markup-insertion/html-unsafe-methods/Document-parseHTMLUnsafe-url-moretests.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel=author href="mailto:jarhar@chromium.org">
<!-- This test was adapted from DOMParser-parseFromString-url-moretests.html -->
<meta charset=utf-8>
<title>Document.parseHTMLUnsafe: Document's url</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id=log></div>
<script>
async_test(function() {
  var iframe = document.createElement("iframe");
  iframe.onload = this.step_func(function() {
    var child = iframe.contentWindow;

    test(function() {
      var doc = Document.parseHTMLUnsafe("<html></html>");
      assert_equals(doc.URL, "about:blank");
    }, "Parent window");

    test(function() {
      var doc = child.Document.parseHTMLUnsafe("<html></html>");
      assert_equals(doc.URL, "about:blank");
    }, "Child window");

    var dpBeforeNavigation = child.Document, urlBeforeNavigation = child.document.URL;
    iframe.onload = this.step_func_done(function() {
      test(function() {
        var doc = dpBeforeNavigation.parseHTMLUnsafe("<html></html>");
        assert_equals(doc.URL, "about:blank");
      }, "Child window crossing navigation");

      test(function() {
        var doc = child.Document.parseHTMLUnsafe("<html></html>");
        assert_equals(doc.URL, "about:blank");
      }, "Child window after navigation");
    });
    iframe.src = "/common/blank.html?2";
  });
  iframe.src = "/common/blank.html?1";
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
  "source_name": "html/webappapis/dynamic-markup-insertion/html-unsafe-methods/Document-parseHTMLUnsafe-url-moretests.html"
}
```
