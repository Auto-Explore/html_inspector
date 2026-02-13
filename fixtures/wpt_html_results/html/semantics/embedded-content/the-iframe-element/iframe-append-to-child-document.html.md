# html/semantics/embedded-content/the-iframe-element/iframe-append-to-child-document.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-iframe-element/iframe-append-to-child-document.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset=utf-8>
<title>Append iframe element to its own child document</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<iframe id=x></iframe>
<script>
test(function() {
  var iframe = document.getElementById('x');
  var childWindow = iframe.contentWindow;
  assert_equals(childWindow.parent, window);
  childWindow.document.body.appendChild(iframe);
  assert_equals(childWindow.parent, null);
  assert_equals(iframe.contentWindow, null);
  assert_equals(childWindow.document.body.firstChild, iframe);
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
  "source_name": "html/semantics/embedded-content/the-iframe-element/iframe-append-to-child-document.html"
}
```
