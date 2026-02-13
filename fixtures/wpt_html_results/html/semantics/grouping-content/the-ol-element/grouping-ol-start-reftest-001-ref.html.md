# html/semantics/grouping-content/the-ol-element/grouping-ol-start-reftest-001-ref.html

Counts:
- errors: 0
- warnings: 18
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/grouping-content/the-ol-element/grouping-ol-start-reftest-001-ref.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <title>ol element</title>
    <link rel="author" title="dzenana" href="mailto:dzenana.trenutak@gmail.com">
    <link rel="help" href="https://html.spec.whatwg.org/multipage/#the-ol-element">
    <meta name="assert" content="Sequences produced by calculated values for LI elements within OL match spec's expectations. (part one)" />
    <style type="text/css">
        span p {display:list-item; margin-left: 0; margin-top: 0; margin-bottom: 0; padding-left: 0; padding-top: 0; padding-bottom: 0;}
        span span p {margin-left: 0; margin-top: 0; margin-bottom: 0; padding-left: 5em; padding-top: 0; padding-bottom: 0; font-family: monospace;}
    </style>
</head>
<body>
    <p>This test continues to validate the ol element. This reftest is necessary because the values of the ol's li children as calculated and displayed by the user agent are NOT systematically available programatically. Only explicitly-set values are available programatically. Therefore, we need to check actual rendering against expected rendering.</p>

    <p><strong>This reftest passes if each list's items are numbered identically to the horizontal sequence immediately above those list items.</strong></p>
    <p>(Note: each list item has no content; only the sequencing should appear.)</p>

    <span>

    <p>2, 3, 4 (ol has start attribute of 2)</p>
    <span>
        <p>2.</p>
        <p>3.</p>
        <p>4.</p>
    </span>

    <p>-9, -8, -7 (ol has start attribute of -9)</p>
    <span>
        <p>-9.</p>
        <p>-8.</p>
        <p>-7.</p>
    </span>

    <p>1000, 1001, 1002 (list's start attribute of 1000 provided by JavaScript)</p>
    <span>
        <p>1000.</p>
        <p>1001.</p>
        <p>1002.</p>
    </span>

    <p>2, 1, 9 (each list item has a specified value attribute, list has a start attribute of 1000)</p>
    <span>
        <p>2.</p>
        <p>1.</p>
        <p>9.</p>
    </span>

    </span>

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
        "byte_end": 420,
        "byte_start": 397,
        "col": 5,
        "line": 9
      }
    },
    {
      "category": "Html",
      "code": "html.p.parent_span",
      "message": "Element “p” not allowed as child of “span” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 1353,
        "byte_start": 1350,
        "col": 5,
        "line": 22
      }
    },
    {
      "category": "Html",
      "code": "html.p.parent_span",
      "message": "Element “p” not allowed as child of “span” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 1417,
        "byte_start": 1414,
        "col": 9,
        "line": 24
      }
    },
    {
      "category": "Html",
      "code": "html.p.parent_span",
      "message": "Element “p” not allowed as child of “span” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 1435,
        "byte_start": 1432,
        "col": 9,
        "line": 25
      }
    },
    {
      "category": "Html",
      "code": "html.p.parent_span",
      "message": "Element “p” not allowed as child of “span” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 1453,
        "byte_start": 1450,
        "col": 9,
        "line": 26
      }
    },
    {
      "category": "Html",
      "code": "html.p.parent_span",
      "message": "Element “p” not allowed as child of “span” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 1480,
        "byte_start": 1477,
        "col": 5,
        "line": 29
      }
    },
    {
      "category": "Html",
      "code": "html.p.parent_span",
      "message": "Element “p” not allowed as child of “span” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 1548,
        "byte_start": 1545,
        "col": 9,
        "line": 31
      }
    },
    {
      "category": "Html",
      "code": "html.p.parent_span",
      "message": "Element “p” not allowed as child of “span” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 1567,
        "byte_start": 1564,
        "col": 9,
        "line": 32
      }
    },
    {
      "category": "Html",
      "code": "html.p.parent_span",
      "message": "Element “p” not allowed as child of “span” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 1586,
        "byte_start": 1583,
        "col": 9,
        "line": 33
      }
    },
    {
      "category": "Html",
      "code": "html.p.parent_span",
      "message": "Element “p” not allowed as child of “span” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 1614,
        "byte_start": 1611,
        "col": 5,
        "line": 36
      }
    },
    {
      "category": "Html",
      "code": "html.p.parent_span",
      "message": "Element “p” not allowed as child of “span” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 1713,
        "byte_start": 1710,
        "col": 9,
        "line": 38
      }
    },
    {
      "category": "Html",
      "code": "html.p.parent_span",
      "message": "Element “p” not allowed as child of “span” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 1734,
        "byte_start": 1731,
        "col": 9,
        "line": 39
      }
    },
    {
      "category": "Html",
      "code": "html.p.parent_span",
      "message": "Element “p” not allowed as child of “span” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 1755,
        "byte_start": 1752,
        "col": 9,
        "line": 40
      }
    },
    {
      "category": "Html",
      "code": "html.p.parent_span",
      "message": "Element “p” not allowed as child of “span” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 1785,
        "byte_start": 1782,
        "col": 5,
        "line": 43
      }
    },
    {
      "category": "Html",
      "code": "html.p.parent_span",
      "message": "Element “p” not allowed as child of “span” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 1904,
        "byte_start": 1901,
        "col": 9,
        "line": 45
      }
    },
    {
      "category": "Html",
      "code": "html.p.parent_span",
      "message": "Element “p” not allowed as child of “span” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 1922,
        "byte_start": 1919,
        "col": 9,
        "line": 46
      }
    },
    {
      "category": "Html",
      "code": "html.p.parent_span",
      "message": "Element “p” not allowed as child of “span” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 1940,
        "byte_start": 1937,
        "col": 9,
        "line": 47
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
  "source_name": "html/semantics/grouping-content/the-ol-element/grouping-ol-start-reftest-001-ref.html"
}
```
