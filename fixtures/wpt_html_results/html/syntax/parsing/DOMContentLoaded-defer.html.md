# html/syntax/parsing/DOMContentLoaded-defer.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/syntax/parsing/DOMContentLoaded-defer.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset=utf-8>
<title>The end: DOMContentLoaded and defer scripts</title>
<link rel=help href="https://html.spec.whatwg.org/multipage/#the-end">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id="log"></div>
<script>
var dcl;
var t = async_test(function() {
  dcl = false;
  document.addEventListener("DOMContentLoaded", function(e) {
    dcl = true;
  });
});
</script>
<script defer src=support/DOMContentLoaded-defer.js></script>
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
  "source_name": "html/syntax/parsing/DOMContentLoaded-defer.html"
}
```
