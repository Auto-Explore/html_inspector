# html/semantics/forms/the-input-element/maxlength-number.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-input-element/maxlength-number.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>input type=number maxlength</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-vendor.js"></script>

<input type="number" maxlength="1">

<script>
  async_test(t => {
    let elem = document.getElementsByTagName("input")[0];
    test_driver.send_keys(elem, "1234")
      .then(t.step_func(() => {
         assert_equals(elem.value, "1234");
         t.done();
      }));
  }, "maxlength doesn't apply to input type=number");
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.input.maxlength.disallowed_for_type",
      "message": "Attribute “maxlength” is only allowed when the input type is “email”, “password”, “search”, “tel”, “text”, or “url”.",
      "severity": "Warning",
      "span": {
        "byte_end": 306,
        "byte_start": 271,
        "col": 1,
        "line": 8
      }
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/semantics/forms/the-input-element/maxlength-number.html"
}
```
