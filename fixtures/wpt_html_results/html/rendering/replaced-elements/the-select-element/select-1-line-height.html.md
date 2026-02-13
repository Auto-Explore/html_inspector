# html/rendering/replaced-elements/the-select-element/select-1-line-height.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/replaced-elements/the-select-element/select-1-line-height.html",
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
  <title>Test: Combobox ignores CSS 'line-height'</title>
  <link rel="author" title="Mats Palmgren" href="mailto:mats@mozilla.com">
  <link rel="match" href="select-1-line-height-ref.html">
  <link rel="help" href="https://html.spec.whatwg.org/multipage/rendering.html#the-select-element-2">
  <link rel="help" href="https://bugzilla.mozilla.org/show_bug.cgi?id=1501908">
  <style type="text/css">
html,body {
  color:black; background-color:white; font:16px/1 monospace;
}

select { line-height:100px; }

  </style>
</head>
<body>

<select><option>aaaaaaaaaa<option>bbbbbbbbbb</select>

<script>
document.body.offsetHeight;
var cv = window.getComputedStyle(document.querySelector('select')).lineHeight;
if (cv != "normal" && parseInt(cv) > 50) {
  document.body.appendChild(document.createTextNode(
    "FAIL: got computed line-height '" + cv + "', " +
    "expected 'normal' or a length <= 50px"));
}</script>

</body>
</html>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.style.type.unnecessary",
      "message": "The “type” attribute for the “style” element is not needed and should be omitted.",
      "severity": "Warning",
      "span": {
        "byte_end": 570,
        "byte_start": 547,
        "col": 3,
        "line": 13
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
  "source_name": "html/rendering/replaced-elements/the-select-element/select-1-line-height.html"
}
```
