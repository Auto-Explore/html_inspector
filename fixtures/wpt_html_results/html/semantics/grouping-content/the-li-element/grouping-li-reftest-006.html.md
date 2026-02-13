# html/semantics/grouping-content/the-li-element/grouping-li-reftest-006.html

Counts:
- errors: 0
- warnings: 7
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/grouping-content/the-li-element/grouping-li-reftest-006.html",
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
  <link rel="help" href="https://bugzilla.mozilla.org/show_bug.cgi?id=1745506">
  <link rel="match" href="grouping-li-reftest-004-ref.html">
  <style>
    body { margin-left: 40px }
    ol { margin: 0; padding: 0; }
    li { list-style-type: decimal }
  </style>
</head>
<body>
<div><ol start=99></ol></div> <!-- this scope shouldn't affect the LIs below -->
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
        "byte_end": 846,
        "byte_start": 842,
        "col": 1,
        "line": 22
      }
    },
    {
      "category": "Html",
      "code": "html.li.parent.disallowed",
      "message": "Element “li” not allowed as child of “body” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 856,
        "byte_start": 852,
        "col": 1,
        "line": 23
      }
    },
    {
      "category": "Html",
      "code": "html.li.parent.disallowed",
      "message": "Element “li” not allowed as child of “body” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 866,
        "byte_start": 862,
        "col": 1,
        "line": 24
      }
    },
    {
      "category": "Html",
      "code": "html.li.parent.disallowed",
      "message": "Element “li” not allowed as child of “body” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 876,
        "byte_start": 872,
        "col": 1,
        "line": 25
      }
    },
    {
      "category": "Html",
      "code": "html.li.parent.disallowed",
      "message": "Element “li” not allowed as child of “div” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 891,
        "byte_start": 887,
        "col": 6,
        "line": 26
      }
    },
    {
      "category": "Html",
      "code": "html.li.parent.disallowed",
      "message": "Element “li” not allowed as child of “div” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 917,
        "byte_start": 913,
        "col": 11,
        "line": 27
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
  "source_name": "html/semantics/grouping-content/the-li-element/grouping-li-reftest-006.html"
}
```
