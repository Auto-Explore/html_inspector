# html/semantics/grouping-content/the-ol-element/grouping-ol-type-reftest-003.html

Counts:
- errors: 0
- warnings: 10
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/grouping-content/the-ol-element/grouping-ol-type-reftest-003.html",
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
    <link rel="match" href="grouping-ol-type-reftest-003-ref.html" />
    <meta name="assert" content="OL's type attribute defaults to decimal state." />
    <style type="text/css">
        span p {display:list-item; margin-left: 0; margin-top: 0; margin-bottom: 0; padding-left: 0; padding-top: 0; padding-bottom: 0;}
        span li {margin-left: 0; margin-top: 0; margin-bottom: 0; padding-left: 0; padding-top: 0; padding-bottom: 0; font-family: monospace;}
        span ol {margin-left: 0; margin-top: 0; margin-bottom: 0; padding-left: 5em; padding-top: 0; padding-bottom: 0; font-family: monospace; list-style-position: inside;}
    </style>
</head>
<body>
    <p>This test continues to validate the ol element. This reftest is necessary because the values of the ol's li children as calculated and displayed by the user agent are NOT systematically available programatically. Only explicitly-set values are available programatically. Therefore, we need to check actual rendering against expected rendering.</p>

    <p><strong>This reftest passes if each list's items are labelled identically to the horizontal sequence immediately above those list items:</strong></p>
    <p>(Note: each list item has no content; only the sequencing should appear.)</p>

    <span>

    <p>-3, -2, -1 (type is "a", start is -3)</p>
    <ol type="a" start="-3">
        <li></li>
        <li></li>
        <li></li>
    </ol>

    <p>0, a (type is "a", start is 0)</p>
    <ol type="a" start="0">
        <li></li>
        <li></li>
    </ol>

    <p>-3, -2, -1 (type is "A", start is -3)</p>
    <ol type="A" start="-3">
        <li></li>
        <li></li>
        <li></li>
    </ol>

    <p>0, A (type is "A", start is 0)</p>
    <ol type="A" start="0">
        <li></li>
        <li></li>
    </ol>

    <p>-3, -2, -1 (type is "i", start is -3)</p>
    <ol type="i" start="-3">
        <li></li>
        <li></li>
        <li></li>
    </ol>

    <p>0, i (type is "i", start is 0)</p>
    <ol type="i" start="0">
        <li></li>
        <li></li>
    </ol>

    <p>-3, -2, -1 (type is "I", start is -3)</p>
    <ol type="I" start="-3">
        <li></li>
        <li></li>
        <li></li>
    </ol>

    <p>0, I (type is "I", start is 0)</p>
    <ol type="I" start="0">
        <li></li>
        <li></li>
    </ol>

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
        "byte_end": 433,
        "byte_start": 410,
        "col": 5,
        "line": 10
      }
    },
    {
      "category": "Html",
      "code": "html.p.parent_span",
      "message": "Element “p” not allowed as child of “span” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 1534,
        "byte_start": 1531,
        "col": 5,
        "line": 24
      }
    },
    {
      "category": "Html",
      "code": "html.p.parent_span",
      "message": "Element “p” not allowed as child of “span” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 1677,
        "byte_start": 1674,
        "col": 5,
        "line": 31
      }
    },
    {
      "category": "Html",
      "code": "html.p.parent_span",
      "message": "Element “p” not allowed as child of “span” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 1794,
        "byte_start": 1791,
        "col": 5,
        "line": 37
      }
    },
    {
      "category": "Html",
      "code": "html.p.parent_span",
      "message": "Element “p” not allowed as child of “span” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 1937,
        "byte_start": 1934,
        "col": 5,
        "line": 44
      }
    },
    {
      "category": "Html",
      "code": "html.p.parent_span",
      "message": "Element “p” not allowed as child of “span” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 2054,
        "byte_start": 2051,
        "col": 5,
        "line": 50
      }
    },
    {
      "category": "Html",
      "code": "html.p.parent_span",
      "message": "Element “p” not allowed as child of “span” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 2197,
        "byte_start": 2194,
        "col": 5,
        "line": 57
      }
    },
    {
      "category": "Html",
      "code": "html.p.parent_span",
      "message": "Element “p” not allowed as child of “span” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 2314,
        "byte_start": 2311,
        "col": 5,
        "line": 63
      }
    },
    {
      "category": "Html",
      "code": "html.p.parent_span",
      "message": "Element “p” not allowed as child of “span” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 2457,
        "byte_start": 2454,
        "col": 5,
        "line": 70
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
  "source_name": "html/semantics/grouping-content/the-ol-element/grouping-ol-type-reftest-003.html"
}
```
