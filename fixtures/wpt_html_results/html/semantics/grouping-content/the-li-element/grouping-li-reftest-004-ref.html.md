# html/semantics/grouping-content/the-li-element/grouping-li-reftest-004-ref.html

Counts:
- errors: 0
- warnings: 7
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/grouping-content/the-li-element/grouping-li-reftest-004-ref.html",
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
  <title>HTML LI element: implied scope</title>
  <link rel="author" title="Mats Palmgren" href="mailto:mats@mozilla.org">
  <style>
    body { margin-left: 40px }
    li { list-style-type: decimal }
  </style>
</head>
<body>
<li start="1"></li>
<li start="2"></li>
<li start="3"></li>
<li start="4"></li>
<div><li></li></div>
<div><div><li></li></div></div>
</body>
</html>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.li.parent.disallowed",
      "message": "Element “li” not allowed as child of “body” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 412,
        "byte_start": 398,
        "col": 1,
        "line": 16
      }
    },
    {
      "category": "Html",
      "code": "html.li.parent.disallowed",
      "message": "Element “li” not allowed as child of “body” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 432,
        "byte_start": 418,
        "col": 1,
        "line": 17
      }
    },
    {
      "category": "Html",
      "code": "html.li.parent.disallowed",
      "message": "Element “li” not allowed as child of “body” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 452,
        "byte_start": 438,
        "col": 1,
        "line": 18
      }
    },
    {
      "category": "Html",
      "code": "html.li.parent.disallowed",
      "message": "Element “li” not allowed as child of “body” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 472,
        "byte_start": 458,
        "col": 1,
        "line": 19
      }
    },
    {
      "category": "Html",
      "code": "html.li.parent.disallowed",
      "message": "Element “li” not allowed as child of “div” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 487,
        "byte_start": 483,
        "col": 6,
        "line": 20
      }
    },
    {
      "category": "Html",
      "code": "html.li.parent.disallowed",
      "message": "Element “li” not allowed as child of “div” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 513,
        "byte_start": 509,
        "col": 11,
        "line": 21
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
  "source_name": "html/semantics/grouping-content/the-li-element/grouping-li-reftest-004-ref.html"
}
```
