# html/rendering/widgets/select-wrap-no-spill.optional.html

Counts:
- errors: 1
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/widgets/select-wrap-no-spill.optional.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<body>
<table>
<tr><td id="target">
<select style="width:80px; white-space:pre-wrap;">
<option>ab option with a very long text that does not fit but should not spill</option>
</select>
<tr><td id="reference"><select style="width:80px;">
<option>ab option with a very long text that does not fit but should not spill</option>
</select>
</table>
<script>
// crbug.com/924929
test(() => {
  assert_equals(document.querySelector('#target').offsetHeight,
      document.querySelector('#reference').offsetHeight);
}, 'Selected OPTION label with white-space:pre-wrap should not spill out.');
</script>
</body>
</html>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.parser.select_in_table",
      "message": "Start tag “select” seen in “table”.",
      "severity": "Error",
      "span": {
        "byte_end": 215,
        "byte_start": 165,
        "col": 1,
        "line": 8
      }
    },
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
  "source_name": "html/rendering/widgets/select-wrap-no-spill.optional.html"
}
```
