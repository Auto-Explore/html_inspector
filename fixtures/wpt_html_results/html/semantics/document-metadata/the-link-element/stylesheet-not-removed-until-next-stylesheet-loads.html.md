# html/semantics/document-metadata/the-link-element/stylesheet-not-removed-until-next-stylesheet-loads.html

Counts:
- errors: 2
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/document-metadata/the-link-element/stylesheet-not-removed-until-next-stylesheet-loads.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel="author" title="Dom Farolino" href="mailto:dom@chromium.org">
<link rel="help" href="https://html.spec.whatwg.org/multipage/links.html#link-type-stylesheet">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<link href="style.css" rel="stylesheet" id="style_test">
<script>
  test(function() {
    assert_true(document.styleSheets.length === 1 &&
                document.styleSheets[0].href.includes("style.css"),
                "The style sheet 'style.css' must be available to scripts");

    style_test.href = "resources/neutral.css?pipe=trickle(d1)";

    assert_true(document.styleSheets.length === 1 &&
                document.styleSheets[0].href.includes("style.css"),
                "The style sheet 'style.css' must remain accessible to " +
                "scripts until its replacement has finished loading");
  }, "Check that a style sheet loaded by a <link> is available until its successor is loaded");
</script>
</head>
</html>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.parser.stray_end_tag",
      "message": "Stray end tag “head”.",
      "severity": "Error",
      "span": {
        "byte_end": 1020,
        "byte_start": 1013,
        "col": 1,
        "line": 21
      }
    },
    {
      "category": "Html",
      "code": "html.parser.stray_end_tag",
      "message": "Stray end tag “html”.",
      "severity": "Error",
      "span": {
        "byte_end": 1028,
        "byte_start": 1021,
        "col": 1,
        "line": 22
      }
    },
    {
      "category": "Html",
      "code": "html.head.title.missing",
      "message": "Element “head” is missing a required instance of child element “title”.",
      "severity": "Warning",
      "span": null
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/semantics/document-metadata/the-link-element/stylesheet-not-removed-until-next-stylesheet-loads.html"
}
```
