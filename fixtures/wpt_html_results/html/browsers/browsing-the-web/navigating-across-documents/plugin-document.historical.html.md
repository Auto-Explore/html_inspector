# html/browsers/browsing-the-web/navigating-across-documents/plugin-document.historical.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/browsing-the-web/navigating-across-documents/plugin-document.historical.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>Same-origin PDFs must not create accessible Document objects</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<!-- https://github.com/whatwg/html/pull/6947 -->

<iframe src="resources/portable-document-format-sample-valid.pdf"></iframe>

<script>
setup({ explicit_done: true });

window.onload = () => {
  test(() => {
    assert_throws_dom("SecurityError", () => {
      document.querySelector("iframe").contentWindow.document;
    });
  });
  done();
};
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
  "source_name": "html/browsers/browsing-the-web/navigating-across-documents/plugin-document.historical.html"
}
```
