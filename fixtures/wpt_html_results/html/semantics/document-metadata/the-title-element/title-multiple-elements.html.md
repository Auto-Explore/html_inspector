# html/semantics/document-metadata/the-title-element/title-multiple-elements.html

Counts:
- errors: 0
- warnings: 0
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/document-metadata/the-title-element/title-multiple-elements.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width">
  <title>Title One</title>
  <title>Title Two</title>
  <link rel="author" title="Deepith N" href="mailto:deepithdeekshith@gmail.com">
  <link rel="help" href="https://html.spec.whatwg.org/multipage/semantics.html#the-title-element">
  <script src="/resources/testharness.js"></script>
  <script src="/resources/testharnessreport.js"></script>
</head>
<body>
  <div id="log"></div>

  <script>
    test(function() {
      assert_equals(document.title, "Title One", "The document's title should be the content of the first <title> element.");
    }, "Browsers must ignore <title> elements after the first one.");
  </script>
</body>
</html>
```

```json
{
  "messages": [],
  "source_name": "html/semantics/document-metadata/the-title-element/title-multiple-elements.html"
}
```
