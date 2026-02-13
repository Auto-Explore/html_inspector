# html/rendering/replaced-elements/the-select-element/select-empty.html

Counts:
- errors: 1
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/replaced-elements/the-select-element/select-empty.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE HTML>
<!--
     Any copyright is dedicated to the Public Domain.
     http://creativecommons.org/publicdomain/zero/1.0/
-->
<html><head>
  <meta charset="utf-8">
  <title>Test: empty SELECT</title>
  <link rel="author" title="Mats Palmgren" href="mailto:mats@mozilla.com">
  <link rel="match" href="select-empty-ref.html">
  <link rel="help" href="https://html.spec.whatwg.org/multipage/rendering.html#the-select-element-2">
  <link rel="help" href="https://bugzilla.mozilla.org/show_bug.cgi?id=1499230">
</head>
<body>

<table border="1" cellpadding="10">
<tr>
<td><select size="4"></select>
<td><select size="4" style="-webkit-appearance: none"></select>
<td><select size="4" style="-webkit-appearance: none; border: 1px solid black"></select>
<td><select size="4" style="border: 1px solid black"></select>
</table>

<table border="1" cellpadding="10">
<tr>
<td><select size="1"></select>
<td><select size="1" style="-webkit-appearance: none"></select>
<td><select size="1" style="-webkit-appearance: none; border: 1px solid black"></select>
<td><select size="1" style="border: 1px solid black"></select>
</table>

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
        "byte_end": 593,
        "byte_start": 576,
        "col": 5,
        "line": 18
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
  "source_name": "html/rendering/replaced-elements/the-select-element/select-empty.html"
}
```
