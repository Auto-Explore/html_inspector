# html/semantics/forms/the-select-element/select-setcustomvalidity.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-select-element/select-setcustomvalidity.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE HTML>
<title>select setCustomValidity</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<select id='select_test'></select>

<script>

test(() => {
  let elem = document.getElementById("select_test");
  assert_false(elem.validity.customError);
  elem.setCustomValidity("custom error");
  assert_true(elem.validity.customError);
}, "select setCustomValidity is correct")

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
  "source_name": "html/semantics/forms/the-select-element/select-setcustomvalidity.html"
}
```
