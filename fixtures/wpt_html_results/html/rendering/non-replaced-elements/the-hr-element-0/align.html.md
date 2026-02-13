# html/rendering/non-replaced-elements/the-hr-element-0/align.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/the-hr-element-0/align.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset="utf-8">
<link rel="match" href="align-ref.html">
<style>
hr {
  width: 100px;
}
</style>

<hr align=>
<hr align=left>
<hr align=center>
<hr align=right>
<hr align=foobar>

<script>
// Test the IDL attribute
const values = ['', 'left', 'center', 'right', 'foobar'];
values.forEach(value => {
  const hr = document.createElement('hr');
  hr.align = value;
  document.body.appendChild(hr);
});
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.tokenizer.attr_value_missing",
      "message": "Attribute value missing.",
      "severity": "Warning",
      "span": {
        "byte_end": 132,
        "byte_start": 121,
        "col": 1,
        "line": 10
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
  "source_name": "html/rendering/non-replaced-elements/the-hr-element-0/align.html"
}
```
