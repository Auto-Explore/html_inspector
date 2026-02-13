# html/semantics/grouping-content/the-ol-element/grouping-ol-type-reftest-001.html

Counts:
- errors: 0
- warnings: 6
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/grouping-content/the-ol-element/grouping-ol-type-reftest-001.html",
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
    <link rel="match" href="grouping-ol-type-reftest-001-ref.html" />
    <meta name="assert" content="OL's type attribute defaults to decimal state." />
    <style type="text/css">
        span p {display:list-item; margin-left: 0; margin-top: 0; margin-bottom: 0; padding-left: 0; padding-top: 0; padding-bottom: 0;}
        span li {margin-left: 0; margin-top: 0; margin-bottom: 0; padding-left: 0; padding-top: 0; padding-bottom: 0; font-family: monospace;}
        span ol {margin-left: 0; margin-top: 0; margin-bottom: 0; padding-left: 5em; padding-top: 0; padding-bottom: 0; font-family: monospace; list-style-position: inside; }
    </style>
</head>
<body>
    <p>This test continues to validate the ol element. This reftest is necessary because the values of the ol's li children as calculated and displayed by the user agent are NOT systematically available programatically. Only explicitly-set values are available programatically. Therefore, we need to check actual rendering against expected rendering.</p>

    <p><strong>This reftest passes if each list's items are labelled identically to the horizontal sequence immediately above those list items:</strong></p>
    <p>(Note: each list item has no content; only the sequencing should appear.)</p>

    <span>

    <p>1, 2, 3 (default value for unspecified type attribute is 'decimal')</p>
    <ol>
        <li></li>
        <li></li>
        <li></li>
    </ol>

    <p>1, 2, 3 (default value for type attribute not listed in spec table is 'decimal' (type = "!"))</p>
    <ol type="!">
        <li></li>
        <li></li>
        <li></li>
    </ol>

    <p>1, 2, 3 (default value for type attribute not listed in spec table is 'decimal' (type = "2"))</p>
    <ol type="2">
        <li></li>
        <li></li>
        <li></li>
    </ol>

    <p>1, 2, 3 (default value for type attribute not listed in spec table is 'decimal' (type = "b"))</p>
    <ol type="b">
        <li></li>
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
        "byte_end": 1535,
        "byte_start": 1532,
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
        "byte_end": 1688,
        "byte_start": 1685,
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
        "byte_end": 1876,
        "byte_start": 1873,
        "col": 5,
        "line": 38
      }
    },
    {
      "category": "Html",
      "code": "html.p.parent_span",
      "message": "Element “p” not allowed as child of “span” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 2064,
        "byte_start": 2061,
        "col": 5,
        "line": 45
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
  "source_name": "html/semantics/grouping-content/the-ol-element/grouping-ol-type-reftest-001.html"
}
```
