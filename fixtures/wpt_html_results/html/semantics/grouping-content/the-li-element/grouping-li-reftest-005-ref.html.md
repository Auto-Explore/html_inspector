# html/semantics/grouping-content/the-li-element/grouping-li-reftest-005-ref.html

Counts:
- errors: 0
- warnings: 4
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/grouping-content/the-li-element/grouping-li-reftest-005-ref.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<!--
     Any copyright is dedicated to the Public Domain.
     http://creativecommons.org/publicdomain/zero/1.0/
-->
<html><head>
  <meta charset="utf-8">
  <title>HTML LI element: explicit scope</title>
  <link rel="author" title="Mats Palmgren" href="mailto:mats@mozilla.org">
</head>
<body>
<ol>
<li start="1"></li>
<li start="2"></li>
<li start="3"></li>
<li start="4"></li>
<div><li start="5"></li></div>
<div><li start="6"></li></div>
<div><div><li start="7"></li></div></div>
</ol>
</body>
</html>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.li.parent.disallowed",
      "message": "Element “li” not allowed as child of “div” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 415,
        "byte_start": 401,
        "col": 6,
        "line": 17
      }
    },
    {
      "category": "Html",
      "code": "html.li.parent.disallowed",
      "message": "Element “li” not allowed as child of “div” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 446,
        "byte_start": 432,
        "col": 6,
        "line": 18
      }
    },
    {
      "category": "Html",
      "code": "html.li.parent.disallowed",
      "message": "Element “li” not allowed as child of “div” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 482,
        "byte_start": 468,
        "col": 11,
        "line": 19
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
  "source_name": "html/semantics/grouping-content/the-li-element/grouping-li-reftest-005-ref.html"
}
```
