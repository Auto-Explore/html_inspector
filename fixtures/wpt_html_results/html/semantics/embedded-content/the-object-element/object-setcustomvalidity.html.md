# html/semantics/embedded-content/the-object-element/object-setcustomvalidity.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-object-element/object-setcustomvalidity.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE HTML>
<title>object setCustomValidity</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<object id='object_test'></object>

<script>

test(() => {
  let elem = document.getElementById("object_test");
  assert_false(elem.validity.customError);
  elem.setCustomValidity("custom error");
  assert_true(elem.validity.customError);
}, "object setCustomValidity is correct")

</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.object.data.missing",
      "message": "Element “object” is missing required attribute “data”.",
      "severity": "Warning",
      "span": {
        "byte_end": 188,
        "byte_start": 163,
        "col": 1,
        "line": 6
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
  "source_name": "html/semantics/embedded-content/the-object-element/object-setcustomvalidity.html"
}
```
