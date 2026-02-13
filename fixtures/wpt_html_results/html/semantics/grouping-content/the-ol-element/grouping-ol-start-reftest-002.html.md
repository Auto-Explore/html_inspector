# html/semantics/grouping-content/the-ol-element/grouping-ol-start-reftest-002.html

Counts:
- errors: 0
- warnings: 6
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/grouping-content/the-ol-element/grouping-ol-start-reftest-002.html",
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
    <link rel="match" href="grouping-ol-start-reftest-002-ref.html" />
    <meta name="assert" content="Sequences produced by calculated values for LI elements within OL match spec's expectations. (part two)" />
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

    <p>4, 5, 6 (ol has start attribute of 2 which is overridden by first list item's value attribute of 4)</p>
    <ol start="2">
        <li value="4"></li>
        <li></li>
        <li></li>
    </ol>

    <p>4, 5, 6 (ol has start attribute of -10 which is overridden by first list item's value attribute of 4)</p>
    <ol start="-10">
        <li value="4"></li>
        <li></li>
        <li></li>
    </ol>

    <p>1, 5, 6 (2nd list item has value attribute of 5)</p>
    <ol>
        <li></li>
        <li value="5"></li>
        <li></li>
    </ol>

    <p>-1, -5, -4 (list has a start attribute of -1, and 2nd list item has value attribute of -5)</p>
    <ol start="-1">
        <li></li>
        <li value="-5"></li>
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
        "byte_end": 1840,
        "byte_start": 1837,
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
        "byte_end": 2049,
        "byte_start": 2046,
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
        "byte_end": 2193,
        "byte_start": 2190,
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
  "source_name": "html/semantics/grouping-content/the-ol-element/grouping-ol-start-reftest-002.html"
}
```
