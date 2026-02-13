# html/semantics/grouping-content/the-ol-element/grouping-ol-type-reftest-002-ref.html

Counts:
- errors: 0
- warnings: 22
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/grouping-content/the-ol-element/grouping-ol-type-reftest-002-ref.html",
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
    <meta name="assert" content="List items are rendered consistently with the state of the type attribute." />
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
        <p>1, 2, 3 (type attribute of "1" results in decimal type)</p>
        <span>
            <p>1.</p>
            <p>2.</p>
            <p>3.</p>
        </span>

        <p>aa, ab, ac (type attribute of "a" results in lower case latin alphabet, start = 27)</p>
        <span>
            <p>aa.</p>
            <p>ab.</p>
            <p>ac.</p>
        </span>

        <p>AA, AB, AC (type attribute of "A" results in upper case latin alphabet, start = 27)</p>
        <span>
            <p>AA.</p>
            <p>AB.</p>
            <p>AC.</p>
        </span>

        <p>i, v, c (type attribute of "i" results in lower case roman alphabet, list values = 1, 5, 100)</p>
        <span>
            <p>i.</p>
            <p>v.</p>
            <p>c.</p>
        </span>

        <p>I, V, C (type attribute of "I" results in upper case roman alphabet, list values = 1, 5, 100)</p>
        <span>
            <p>I.</p>
            <p>V.</p>
            <p>C.</p>
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
        "byte_end": 391,
        "byte_start": 368,
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
        "byte_end": 1327,
        "byte_start": 1324,
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
        "byte_end": 1417,
        "byte_start": 1414,
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
        "byte_end": 1439,
        "byte_start": 1436,
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
        "byte_end": 1461,
        "byte_start": 1458,
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
        "byte_end": 1496,
        "byte_start": 1493,
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
        "byte_end": 1614,
        "byte_start": 1611,
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
        "byte_end": 1637,
        "byte_start": 1634,
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
        "byte_end": 1660,
        "byte_start": 1657,
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
        "byte_end": 1696,
        "byte_start": 1693,
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
        "byte_end": 1814,
        "byte_start": 1811,
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
        "byte_end": 1860,
        "byte_start": 1857,
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
        "byte_end": 1896,
        "byte_start": 1893,
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
        "byte_end": 2024,
        "byte_start": 2021,
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
        "byte_end": 2046,
        "byte_start": 2043,
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
        "byte_end": 2068,
        "byte_start": 2065,
        "col": 13,
        "line": 46
      }
    },
    {
      "category": "Html",
      "code": "html.p.parent_span",
      "message": "Element “p” not allowed as child of “span” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 2103,
        "byte_start": 2100,
        "col": 9,
        "line": 49
      }
    },
    {
      "category": "Html",
      "code": "html.p.parent_span",
      "message": "Element “p” not allowed as child of “span” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 2231,
        "byte_start": 2228,
        "col": 13,
        "line": 51
      }
    },
    {
      "category": "Html",
      "code": "html.p.parent_span",
      "message": "Element “p” not allowed as child of “span” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 2253,
        "byte_start": 2250,
        "col": 13,
        "line": 52
      }
    },
    {
      "category": "Html",
      "code": "html.p.parent_span",
      "message": "Element “p” not allowed as child of “span” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 2275,
        "byte_start": 2272,
        "col": 13,
        "line": 53
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
  "source_name": "html/semantics/grouping-content/the-ol-element/grouping-ol-type-reftest-002-ref.html"
}
```
