# html/browsers/browsing-the-web/navigating-across-documents/javascript-url-global-scope.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/browsing-the-web/navigating-across-documents/javascript-url-global-scope.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset=utf-8>
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>

<a id="javascript-link" href="javascript:changeStatus()">link</a>

<script>
function changeStatus() {
  t.done();
}

var t = async_test(function(t) {
  document.querySelector("#javascript-link").click();
}, "javascript: scheme urls should be executed in current global scope");
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.head.title.missing",
      "message": "Element “head” is missing a required instance of child element “title”.",
      "severity": "Warning",
      "span": null
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/browsers/browsing-the-web/navigating-across-documents/javascript-url-global-scope.html"
}
```
