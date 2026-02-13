# html/semantics/grouping-content/the-li-element/grouping-li-reftest-004.html

Counts:
- errors: 0
- warnings: 7
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/grouping-content/the-li-element/grouping-li-reftest-004.html",
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
  <link rel="help" href="https://html.spec.whatwg.org/multipage/grouping-content.html#the-li-element">
  <link rel="help" href="https://drafts.csswg.org/css-lists/#propdef-counter-reset">
  <link rel="match" href="grouping-li-reftest-004-ref.html">
  <style>
    body { margin-left: 40px }
    li { list-style-type: decimal }
  </style>
</head>
<body>
<li></li>
<li></li>
<li></li>
<li></li>
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
        "byte_end": 651,
        "byte_start": 647,
        "col": 1,
        "line": 19
      }
    },
    {
      "category": "Html",
      "code": "html.li.parent.disallowed",
      "message": "Element “li” not allowed as child of “body” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 661,
        "byte_start": 657,
        "col": 1,
        "line": 20
      }
    },
    {
      "category": "Html",
      "code": "html.li.parent.disallowed",
      "message": "Element “li” not allowed as child of “body” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 671,
        "byte_start": 667,
        "col": 1,
        "line": 21
      }
    },
    {
      "category": "Html",
      "code": "html.li.parent.disallowed",
      "message": "Element “li” not allowed as child of “body” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 681,
        "byte_start": 677,
        "col": 1,
        "line": 22
      }
    },
    {
      "category": "Html",
      "code": "html.li.parent.disallowed",
      "message": "Element “li” not allowed as child of “div” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 696,
        "byte_start": 692,
        "col": 6,
        "line": 23
      }
    },
    {
      "category": "Html",
      "code": "html.li.parent.disallowed",
      "message": "Element “li” not allowed as child of “div” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 722,
        "byte_start": 718,
        "col": 11,
        "line": 24
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
  "source_name": "html/semantics/grouping-content/the-li-element/grouping-li-reftest-004.html"
}
```
