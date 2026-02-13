# html/semantics/grouping-content/the-li-element/grouping-li-reftest-007.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/grouping-content/the-li-element/grouping-li-reftest-007.html",
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
    ol { margin: 0; padding: 0 }
    li { list-style-type: decimal }
  </style>
</head>
<body>
<div><ol start=99></ol></div> <!-- this scope shouldn't affect the LIs below -->
<ol style="counter-reset: item"> <!-- this counter shouldn't affect the LIs below -->
<li></li>
<li></li>
<li></li>
<li></li>
<div><li></li></div>
<div><div><li></li></div></div>
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
        "byte_end": 976,
        "byte_start": 972,
        "col": 6,
        "line": 27
      }
    },
    {
      "category": "Html",
      "code": "html.li.parent.disallowed",
      "message": "Element “li” not allowed as child of “div” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 1002,
        "byte_start": 998,
        "col": 11,
        "line": 28
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
  "source_name": "html/semantics/grouping-content/the-li-element/grouping-li-reftest-007.html"
}
```
