# html/semantics/grouping-content/the-figure-element/grouping-figure.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/grouping-content/the-figure-element/grouping-figure.html",
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
    <title>the figure element</title>
    <link rel="author" title="dzenana" href="mailto:dzenana.trenutak@gmail.com">
    <link rel="help" href="https://html.spec.whatwg.org/multipage/#the-figure-element">
    <script src="/resources/testharness.js"></script>
    <script src="/resources/testharnessreport.js"></script>
    <script>
        "use strict";

        // check that prototype matches spec's DOM interface
        test(function () {
            var testElement = document.createElement("figure");
            assert_equals(Object.getPrototypeOf(testElement), HTMLElement.prototype, "HTMLElement.prototype should be used for figure");
        }, "The prototype for figure is HTMLElement.prototype");

        // Not checking: Sectioning root

    </script>
</head>
<body>
    <h1>Description</h1>
    <p>This test validates the figure element.</p>

    <div id="log"></div>
</body>
</html>
```

```json
{
  "messages": [
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/semantics/grouping-content/the-figure-element/grouping-figure.html"
}
```
