# html/semantics/grouping-content/the-ol-element/grouping-ol-type-reftest-001-ref.html

Counts:
- errors: 0
- warnings: 18
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/grouping-content/the-ol-element/grouping-ol-type-reftest-001-ref.html",
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
    <meta name="assert" content="OL's type attribute defaults to decimal state." />
    <style type="text/css">
        span p {display:list-item; margin-left: 0; margin-top: 0; margin-bottom: 0; padding-left: 0; padding-top: 0; padding-bottom: 0;}
        span span p {margin-left: 0; margin-top: 0; margin-bottom: 0; padding-left: 5em; padding-top: 0; padding-bottom: 0; font-family: monospace;}
    </style>
</head>
<body>
    <p>This test continues to validate the ol element. This reftest is necessary because the values of the ol's li children as calculated and displayed by the user agent are NOT systematically available programatically. Only explicitly-set values are available programatically. Therefore, we need to check actual rendering against expected rendering.</p>

    <p><strong>This reftest passes if each list's items are labelled identically to the horizontal sequence immediately above those list items:</strong></p>
    <p>(Note: each list item has no content; only the sequencing should appear.)</p>

    <span>
        <p>1, 2, 3 (default value for unspecified type attribute is 'decimal')</p>
        <span>
            <p>1.</p>
            <p>2.</p>
            <p>3.</p>
        </span>

        <p>1, 2, 3 (default value for type attribute not listed in spec table is 'decimal' (type = "!"))</p>
        <span>
            <p>1.</p>
            <p>2.</p>
            <p>3.</p>
        </span>

        <p>1, 2, 3 (default value for type attribute not listed in spec table is 'decimal' (type = "2"))</p>
        <span>
            <p>1.</p>
            <p>2.</p>
            <p>3.</p>
        </span>

        <p>1, 2, 3 (default value for type attribute not listed in spec table is 'decimal' (type = "b"))</p>
        <span>
            <p>1.</p>
            <p>2.</p>
            <p>3.</p>
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
        "byte_end": 363,
        "byte_start": 340,
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
        "byte_end": 1299,
        "byte_start": 1296,
        "col": 9,
        "line": 21
      }
    },
    {
      "category": "Html",
      "code": "html.p.parent_span",
      "message": "Element “p” not allowed as child of “span” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 1401,
        "byte_start": 1398,
        "col": 13,
        "line": 23
      }
    },
    {
      "category": "Html",
      "code": "html.p.parent_span",
      "message": "Element “p” not allowed as child of “span” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 1423,
        "byte_start": 1420,
        "col": 13,
        "line": 24
      }
    },
    {
      "category": "Html",
      "code": "html.p.parent_span",
      "message": "Element “p” not allowed as child of “span” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 1445,
        "byte_start": 1442,
        "col": 13,
        "line": 25
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
        "col": 9,
        "line": 28
      }
    },
    {
      "category": "Html",
      "code": "html.p.parent_span",
      "message": "Element “p” not allowed as child of “span” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 1608,
        "byte_start": 1605,
        "col": 13,
        "line": 30
      }
    },
    {
      "category": "Html",
      "code": "html.p.parent_span",
      "message": "Element “p” not allowed as child of “span” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 1630,
        "byte_start": 1627,
        "col": 13,
        "line": 31
      }
    },
    {
      "category": "Html",
      "code": "html.p.parent_span",
      "message": "Element “p” not allowed as child of “span” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 1652,
        "byte_start": 1649,
        "col": 13,
        "line": 32
      }
    },
    {
      "category": "Html",
      "code": "html.p.parent_span",
      "message": "Element “p” not allowed as child of “span” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 1687,
        "byte_start": 1684,
        "col": 9,
        "line": 35
      }
    },
    {
      "category": "Html",
      "code": "html.p.parent_span",
      "message": "Element “p” not allowed as child of “span” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 1815,
        "byte_start": 1812,
        "col": 13,
        "line": 37
      }
    },
    {
      "category": "Html",
      "code": "html.p.parent_span",
      "message": "Element “p” not allowed as child of “span” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 1837,
        "byte_start": 1834,
        "col": 13,
        "line": 38
      }
    },
    {
      "category": "Html",
      "code": "html.p.parent_span",
      "message": "Element “p” not allowed as child of “span” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 1859,
        "byte_start": 1856,
        "col": 13,
        "line": 39
      }
    },
    {
      "category": "Html",
      "code": "html.p.parent_span",
      "message": "Element “p” not allowed as child of “span” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 1894,
        "byte_start": 1891,
        "col": 9,
        "line": 42
      }
    },
    {
      "category": "Html",
      "code": "html.p.parent_span",
      "message": "Element “p” not allowed as child of “span” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 2022,
        "byte_start": 2019,
        "col": 13,
        "line": 44
      }
    },
    {
      "category": "Html",
      "code": "html.p.parent_span",
      "message": "Element “p” not allowed as child of “span” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 2044,
        "byte_start": 2041,
        "col": 13,
        "line": 45
      }
    },
    {
      "category": "Html",
      "code": "html.p.parent_span",
      "message": "Element “p” not allowed as child of “span” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 2066,
        "byte_start": 2063,
        "col": 13,
        "line": 46
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
  "source_name": "html/semantics/grouping-content/the-ol-element/grouping-ol-type-reftest-001-ref.html"
}
```
