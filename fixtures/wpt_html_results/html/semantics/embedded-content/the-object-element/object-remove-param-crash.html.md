# html/semantics/embedded-content/the-object-element/object-remove-param-crash.html

Counts:
- errors: 1
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-object-element/object-remove-param-crash.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>HTML Test: object - crash removing a param after changing its style</title>
<link rel="help" href="https://crbug.com/1195633">
<object type="text/html">
  <param id="param"></param>
</object>
<script>
  getComputedStyle(param).color;
  param.style.color = "red";
  param.remove();
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
        "byte_end": 175,
        "byte_start": 150,
        "col": 1,
        "line": 4
      }
    },
    {
      "category": "Html",
      "code": "html.element.param.obsolete",
      "message": "The “param” element is obsolete. Use the “data” attribute of the “object” element to set the URL of the external resource.",
      "severity": "Warning",
      "span": {
        "byte_end": 196,
        "byte_start": 178,
        "col": 3,
        "line": 5
      }
    },
    {
      "category": "Html",
      "code": "html.parser.stray_end_tag",
      "message": "Stray end tag “param”.",
      "severity": "Error",
      "span": {
        "byte_end": 204,
        "byte_start": 196,
        "col": 21,
        "line": 5
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
  "source_name": "html/semantics/embedded-content/the-object-element/object-remove-param-crash.html"
}
```
