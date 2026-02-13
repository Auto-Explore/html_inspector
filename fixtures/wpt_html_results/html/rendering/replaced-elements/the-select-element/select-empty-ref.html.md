# html/rendering/replaced-elements/the-select-element/select-empty-ref.html

Counts:
- errors: 1
- warnings: 6
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/replaced-elements/the-select-element/select-empty-ref.html",
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
  <title>Reference: empty SELECT</title>
  <link rel="author" title="Mats Palmgren" href="mailto:mats@mozilla.com">
  <style>

.none { display: none; }

  </style>
</head>
<body>

<table border="1" cellpadding="10">
<tr>
<td><select size="4"><option class="none"></select>
<td><select size="4" style="-webkit-appearance: none"><option class="none">option</select>
<td><select size="4" style="-webkit-appearance: none; border: 1px solid black"><option class="none">option</select>
<td><select size="4" style="border: 1px solid black"><option class="none">option</select>
</table>

<table border="1" cellpadding="10">
<tr>
<td><select size="1"><option class="none"></select>
<td><select size="1" style="-webkit-appearance: none"><option class="none"></select>
<td><select size="1" style="-webkit-appearance: none; border: 1px solid black"><option class="none"></select>
<td><select size="1" style="border: 1px solid black"><option class="none"></select>
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
        "byte_end": 414,
        "byte_start": 397,
        "col": 5,
        "line": 20
      }
    },
    {
      "category": "Html",
      "code": "html.option.empty_without_label",
      "message": "Element “option” without attribute “label” must not be empty.",
      "severity": "Warning",
      "span": null
    },
    {
      "category": "Html",
      "code": "html.option.empty_without_label",
      "message": "Element “option” without attribute “label” must not be empty.",
      "severity": "Warning",
      "span": null
    },
    {
      "category": "Html",
      "code": "html.option.empty_without_label",
      "message": "Element “option” without attribute “label” must not be empty.",
      "severity": "Warning",
      "span": null
    },
    {
      "category": "Html",
      "code": "html.option.empty_without_label",
      "message": "Element “option” without attribute “label” must not be empty.",
      "severity": "Warning",
      "span": null
    },
    {
      "category": "Html",
      "code": "html.option.empty_without_label",
      "message": "Element “option” without attribute “label” must not be empty.",
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
  "source_name": "html/rendering/replaced-elements/the-select-element/select-empty-ref.html"
}
```
