# html/semantics/grouping-content/the-ol-element/grouping-ol-start-reftest-001.html

Counts:
- errors: 0
- warnings: 6
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/grouping-content/the-ol-element/grouping-ol-start-reftest-001.html",
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
    <link rel="match" href="grouping-ol-start-reftest-001-ref.html" />
    <meta name="assert" content="Sequences produced by calculated values for LI elements within OL match spec's expectations (part one)." />
    <style type="text/css">
        span p {display:list-item; margin-left: 0; margin-top: 0; margin-bottom: 0; padding-left: 0; padding-top: 0; padding-bottom: 0;}
        span li {margin-left: 0; margin-top: 0; margin-bottom: 0; padding-left: 0; padding-top: 0; padding-bottom: 0; font-family: monospace;}
        span ol {margin-left: 0; margin-top: 0; margin-bottom: 0; padding-left: 5em; padding-top: 0; padding-bottom: 0; font-family: monospace;
                list-style-position: inside; list-style-type: decimal; }
    </style>
</head>
<body>
    <p>This test continues to validate the ol element. This reftest is necessary because the values of the ol's li children as calculated and displayed by the user agent are NOT systematically available programatically. Only explicitly-set values are available programatically. Therefore, we need to check actual rendering against expected rendering.</p>

    <p><strong>This reftest passes if each list's items are numbered identically to the horizontal sequence immediately above those list items.</strong></p>
    <p>(Note: each list item has no content; only the sequencing should appear.)</p>

    <span>

    <p>2, 3, 4 (ol has start attribute of 2)</p>
    <ol start="2">
        <li></li>
        <li></li>
        <li></li>
    </ol>

    <p>-9, -8, -7 (ol has start attribute of -9)</p>
    <ol start="-9">
        <li></li>
        <li></li>
        <li></li>
    </ol>

    <p>1000, 1001, 1002 (list's start attribute of 1000 provided by JavaScript)</p>
    <ol id="start_me">
        <li></li>
        <li></li>
        <li></li>
    </ol>

    <p>2, 1, 9 (each list item has a specified value attribute, list has a start attribute of 1000)</p>
    <ol istart="1000">
        <li value="2"></li>
        <li value="1"></li>
        <li value="9"></li>
    </ol>

    </span>

    <script>
        document.getElementById("start_me").start = 1000;
    </script>

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
        "byte_end": 491,
        "byte_start": 468,
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
        "byte_end": 1635,
        "byte_start": 1632,
        "col": 5,
        "line": 25
      }
    },
    {
      "category": "Html",
      "code": "html.p.parent_span",
      "message": "Element “p” not allowed as child of “span” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 1768,
        "byte_start": 1765,
        "col": 5,
        "line": 32
      }
    },
    {
      "category": "Html",
      "code": "html.p.parent_span",
      "message": "Element “p” not allowed as child of “span” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 1906,
        "byte_start": 1903,
        "col": 5,
        "line": 39
      }
    },
    {
      "category": "Html",
      "code": "html.p.parent_span",
      "message": "Element “p” not allowed as child of “span” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 2078,
        "byte_start": 2075,
        "col": 5,
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
  "source_name": "html/semantics/grouping-content/the-ol-element/grouping-ol-start-reftest-001.html"
}
```
