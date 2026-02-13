# html/semantics/forms/the-input-element/valueMode-weekmonth.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-input-element/valueMode-weekmonth.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset=utf-8>
<title>Input element value mode</title>
<link rel="author" title="Denis Ah-Kang" href="mailto:denis@w3.org">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id="log"></div>
<script>
// MODE DEFAULT
test(function () {
  var input = document.createElement("input");
  input.type = "month";
  input.value = "foo\r\r\n\n\0";
  assert_equals(input.value, "");
}, "value IDL attribute of input type month without value attribute");
test(function() {
  var input = document.createElement("input");
  input.type = "month";
  input.setAttribute("value", "bar");
  input.value = "foo\r\r\n\n\0";
  assert_equals(input.value, "");
}, "value IDL attribute of input type month with value attribute");

test(function () {
  var input = document.createElement("input");
  input.type = "week";
  input.value = "foo\r\r\n\n\0";
  assert_equals(input.value, "");
}, "value IDL attribute of input type week without value attribute");
test(function() {
  var input = document.createElement("input");
  input.type = "week";
  input.setAttribute("value", "bar");
  input.value = "foo\r\r\n\n\0";
  assert_equals(input.value, "");
}, "value IDL attribute of input type week with value attribute");
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
  "source_name": "html/semantics/forms/the-input-element/valueMode-weekmonth.html"
}
```
