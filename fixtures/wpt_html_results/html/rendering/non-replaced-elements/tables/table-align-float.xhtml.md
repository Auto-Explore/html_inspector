# html/rendering/non-replaced-elements/tables/table-align-float.xhtml

Counts:
- errors: 1
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/tables/table-align-float.xhtml",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE html PUBLIC "-//W3C//DTD XHTML 1.0 Strict//EN" "http://www.w3.org/TR/xhtml1/DTD/xhtml1-strict.dtd">
<html xmlns="http://www.w3.org/1999/xhtml" xml:lang="en" lang="en">
<head>
  <meta charset="utf-8"/>
  <title>Table align attribute test</title>
  <link rel="help" href="https://html.spec.whatwg.org/#tables-2"/>
  <link rel="match" href="table-align-float-ref.xhtml"/>
  <style type="text/css">
    table { border: 1px solid black; width: 300px; }
  </style>
</head>
<body>
  <table align="LEFT">
    <tr><td>Left aligned table</td></tr>
  </table>
  <br/>
  <table align="CENTER">
    <tr><td>Center aligned table</td></tr>
  </table>
  <br/>
  <table align="RIGHT">
    <tr><td>Right aligned table</td></tr>
  </table>
</body>
</html>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.parser.doctype.not_html5",
      "message": "Obsolete doctype. Expected “<!DOCTYPE html>”.",
      "severity": "Error",
      "span": {
        "byte_end": 41,
        "byte_start": 39,
        "col": 1,
        "line": 2
      }
    },
    {
      "category": "Html",
      "code": "html.style.type.unnecessary",
      "message": "The “type” attribute for the “style” element is not needed and should be omitted.",
      "severity": "Warning",
      "span": {
        "byte_end": 443,
        "byte_start": 420,
        "col": 3,
        "line": 9
      }
    }
  ],
  "source_name": "html/rendering/non-replaced-elements/tables/table-align-float.xhtml"
}
```
