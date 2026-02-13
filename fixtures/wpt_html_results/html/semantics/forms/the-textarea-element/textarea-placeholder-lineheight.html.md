# html/semantics/forms/the-textarea-element/textarea-placeholder-lineheight.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-textarea-element/textarea-placeholder-lineheight.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>

<head>
  <title>textarea placeholder line-height</title>
  <link rel="author" title="Daniel Libby" href="mailto:dlibby@microsoft.com">
  <script src="/resources/testharness.js"></script>
  <script src="/resources/testharnessreport.js"></script>
  <style>
    textarea {
      margin: 0;
      border: 0;
      padding: 0;
    }
  </style>
</head>

<body>
  <textarea rows=1 placeholder=foo style="border:0"></textarea>
  <script>
    let textarea = document.querySelector('textarea');
    const lineHeight = 19.5;
    textarea.style.lineHeight = lineHeight + "px";
    test(
      function () {
        assert_equals(textarea.getBoundingClientRect().height, lineHeight);
      }, "Bounding rect height for textarea must be the same as line-height");

    test(
      function () {
        assert_equals(getComputedStyle(textarea).lineHeight, lineHeight + "px");
      }, "ComputedStyle line-height for textarea must be the same as set value");
  </script>
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
  "source_name": "html/semantics/forms/the-textarea-element/textarea-placeholder-lineheight.html"
}
```
