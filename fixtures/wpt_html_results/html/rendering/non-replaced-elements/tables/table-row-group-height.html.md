# html/rendering/non-replaced-elements/tables/table-row-group-height.html

Counts:
- errors: 3
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/tables/table-row-group-height.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<link rel="match" href="table-row-group-height-ref.html">
<table style="border: 1px solid red">
  <thead style="display: block" height="100">
    <tr>
      <td>
        thead text
      </td>
    </tr>
  </tr>
</table>

<table style="border: 1px solid red">
  <tbody style="display: block" height="100">
    <tr>
      <td>
        tbody text
      </td>
    </tr>
  </tr>
</table>

<table style="border: 1px solid red">
  <tfoot style="display: block" height="100">
    <tr>
      <td>
        tfoot text
      </td>
    </tr>
  </tr>
</table>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.parser.stray_end_tag",
      "message": "Stray end tag “tr”.",
      "severity": "Error",
      "span": {
        "byte_end": 226,
        "byte_start": 221,
        "col": 3,
        "line": 10
      }
    },
    {
      "category": "Html",
      "code": "html.parser.stray_end_tag",
      "message": "Stray end tag “tr”.",
      "severity": "Error",
      "span": {
        "byte_end": 389,
        "byte_start": 384,
        "col": 3,
        "line": 20
      }
    },
    {
      "category": "Html",
      "code": "html.parser.stray_end_tag",
      "message": "Stray end tag “tr”.",
      "severity": "Error",
      "span": {
        "byte_end": 552,
        "byte_start": 547,
        "col": 3,
        "line": 30
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
  "source_name": "html/rendering/non-replaced-elements/tables/table-row-group-height.html"
}
```
