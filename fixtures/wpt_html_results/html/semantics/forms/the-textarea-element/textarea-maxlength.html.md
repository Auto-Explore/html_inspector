# html/semantics/forms/the-textarea-element/textarea-maxlength.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-textarea-element/textarea-maxlength.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>

<head>
  <title>textarea maxlength</title>
  <link rel="author" title="tigercosmos" href="mailto:phy.tiger@gmail.com">
  <link rel=help href="https://html.spec.whatwg.org/multipage/#attr-textarea-maxlength">
  <script src="/resources/testharness.js"></script>
  <script src="/resources/testharnessreport.js"></script>
</head>

<body>

  <textarea id="none"></textarea>
  <textarea id="negative" maxlength="-5"></textarea>
  <textarea id="non-numeric" maxlength="not-a-number"></textarea>
  <textarea id="assign-negative"></textarea>
  <textarea id="assign-non-numeric"></textarea>

  <script>
    test(
      function () {
        assert_equals(document.getElementById("none").maxLength, -1);
      }, "Unset maxlength is -1");

    test(
      function () {
        assert_equals(document.getElementById("negative").maxLength, -1);
      }, "Negative maxlength is always -1");

    test(
      function () {
        assert_equals(document.getElementById("non-numeric").maxLength, -1);
      }, "Non-numeric maxlength is -1");

    test(
      function () {
        assert_throws_dom("INDEX_SIZE_ERR", function () {
          document.getElementById("assign-negative").maxLength = -5;
        });
      }, "Assigning negative integer throws IndexSizeError");

    test(
      function () {
        document.getElementById("assign-non-numeric").maxLength = "not-a-number";
        assert_equals(document.getElementById("assign-non-numeric").maxLength, 0);
      }, "Assigning non-numeric to maxlength sets maxlength to 0");
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
  "source_name": "html/semantics/forms/the-textarea-element/textarea-maxlength.html"
}
```
