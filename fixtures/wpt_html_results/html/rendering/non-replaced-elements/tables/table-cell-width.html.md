# html/rendering/non-replaced-elements/tables/table-cell-width.html

Counts:
- errors: 1
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/tables/table-cell-width.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<link rel="match" href="table-cell-width-ref.html">
<style>
body {
  margin: 0;
}

table {
  width: 400px;
  border-collapse: collapse;
}

th {
  font-weight: normal;
  text-align: left;
}

td, th {
  padding: 0;
}

td:first-child, th:first-child {
  background-color: red;
}
</style>

<!-- width=0 should be treated as 'auto' -->
<table>
  <tr>
    <th width=0>a</th>
    <th>a</th>
  </tr>
</table>

<table>
  <tr>
    <td width=0>a</td>
    <td>a</td>
  </tr>
</table>

<!-- test valid width attribute value-->
<table>
  <tr>
    <th width=100>a</th>
    <th>a</th>
  </tr>
</table>

<table>
  <tr>
    <td width=100>a</td>
    <td>a</td>
  </tr>
</table>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.parser.start_tag_without_doctype",
      "message": "Start tag seen without seeing a doctype first. Expected “<!DOCTYPE html>”.",
      "severity": "Error",
      "span": {
        "byte_end": 51,
        "byte_start": 0,
        "col": 1,
        "line": 1
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
  "source_name": "html/rendering/non-replaced-elements/tables/table-cell-width.html"
}
```
