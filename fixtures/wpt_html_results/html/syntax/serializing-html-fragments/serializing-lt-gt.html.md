# html/syntax/serializing-html-fragments/serializing-lt-gt.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/syntax/serializing-html-fragments/serializing-lt-gt.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Escape "&lt;" and ">" in attribute values when serializing</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id="log"></div>

<span id="test"><a b="<>"></a></span>

<script>
const test_el = document.getElementById("test");

test(() => {
  assert_equals(test_el.innerHTML, "<a b=\"&lt;&gt;\"></a>");
}, "innerHTML");

test(() => {
  assert_equals(test_el.outerHTML, "<span id=\"test\"><a b=\"&lt;&gt;\"></a></span>");
}, "outerHTML");
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
  "source_name": "html/syntax/serializing-html-fragments/serializing-lt-gt.html"
}
```
