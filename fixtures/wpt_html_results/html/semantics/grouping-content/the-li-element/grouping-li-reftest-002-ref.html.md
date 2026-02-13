# html/semantics/grouping-content/the-li-element/grouping-li-reftest-002-ref.html

Counts:
- errors: 0
- warnings: 6
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/grouping-content/the-li-element/grouping-li-reftest-002-ref.html",
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
    <title>li element</title>
    <link rel="author" title="dzenana" href="mailto:dzenana.trenutak@gmail.com">
    <link rel="help" href="https://html.spec.whatwg.org/multipage/#the-li-element">
    <meta name="assert" content="li elements with ol parents have ordinal values." />
    <style type="text/css">
        span p {display:list-item; margin-left: 0; margin-top: 0; margin-bottom: 0; padding-left: 0; padding-top: 0; padding-bottom: 0;}
        span span p {margin-left: 0; margin-top: 0; margin-bottom: 0; padding-left: 5em; padding-top: 0; padding-bottom: 0; font-family: monospace;}
    </style>
</head>
<body>
    <h1>Description</h1>
    <p>This test continues to validate the li element.</p>

    <p>This reftest verifies that the value attribute has an effect when applied to a list item with an ol parent.</p>
    <p>A reftest is necessary because the values of li elements as calculated by the user agent are NOT available programatically.  Only explicitly-set values are then available programatically.</p>
    <p>This reftest passes if you see the numbers 1. 2. 3. below the words "Ordered List"</p>

    <span>
        <p>Ordered List</p>
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
        "byte_end": 365,
        "byte_start": 342,
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
        "byte_end": 1200,
        "byte_start": 1197,
        "col": 9,
        "line": 23
      }
    },
    {
      "category": "Html",
      "code": "html.p.parent_span",
      "message": "Element “p” not allowed as child of “span” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 1247,
        "byte_start": 1244,
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
        "byte_end": 1269,
        "byte_start": 1266,
        "col": 13,
        "line": 26
      }
    },
    {
      "category": "Html",
      "code": "html.p.parent_span",
      "message": "Element “p” not allowed as child of “span” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 1291,
        "byte_start": 1288,
        "col": 13,
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
  "source_name": "html/semantics/grouping-content/the-li-element/grouping-li-reftest-002-ref.html"
}
```
