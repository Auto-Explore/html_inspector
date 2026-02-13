# html/semantics/grouping-content/the-ol-element/grouping-ol-start-reftest-002-ref.html

Counts:
- errors: 0
- warnings: 18
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/grouping-content/the-ol-element/grouping-ol-start-reftest-002-ref.html",
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
    <meta name="assert" content="Sequences produced by calculated values for LI elements within OL match spec's expectations. (part two)" />
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

    <p>4, 5, 6 (ol has start attribute of 2 which is overridden by first list item's value attribute of 4)</p>
    <span>
        <p>4.</p>
        <p>5.</p>
        <p>6.</p>
    </span>

    <p>4, 5, 6 (ol has start attribute of -10 which is overridden by first list item's value attribute of 4)</p>
    <span>
        <p>4.</p>
        <p>5.</p>
        <p>6.</p>
    </span>

    <p>1, 5, 6 (2nd list item has value attribute of 5)</p>
    <span>
        <p>1.</p>
        <p>5.</p>
        <p>6.</p>
    </span>

    <p>-1, -5, -4 (list has a start attribute of -1, and 2nd list item has value attribute of -5)</p>
    <span>
        <p>-1.</p>
        <p>-5.</p>
        <p>-4.</p>
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
        "byte_end": 1479,
        "byte_start": 1476,
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
        "byte_end": 1497,
        "byte_start": 1494,
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
        "byte_end": 1515,
        "byte_start": 1512,
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
        "byte_end": 1542,
        "byte_start": 1539,
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
        "byte_end": 1670,
        "byte_start": 1667,
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
        "byte_end": 1688,
        "byte_start": 1685,
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
        "byte_end": 1706,
        "byte_start": 1703,
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
        "byte_end": 1733,
        "byte_start": 1730,
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
        "byte_end": 1808,
        "byte_start": 1805,
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
        "byte_end": 1826,
        "byte_start": 1823,
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
        "byte_end": 1844,
        "byte_start": 1841,
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
        "byte_end": 1871,
        "byte_start": 1868,
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
        "byte_end": 1988,
        "byte_start": 1985,
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
        "byte_end": 2007,
        "byte_start": 2004,
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
        "byte_end": 2026,
        "byte_start": 2023,
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
  "source_name": "html/semantics/grouping-content/the-ol-element/grouping-ol-start-reftest-002-ref.html"
}
```
