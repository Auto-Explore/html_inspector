# html/semantics/forms/the-textarea-element/textarea-minlength.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-textarea-element/textarea-minlength.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>

<head>
  <title>textarea minlength</title>
  <link rel="author" title="tigercosmos" href="mailto:phy.tiger@gmail.com">
  <link rel=help href="https://html.spec.whatwg.org/multipage/#attr-textarea-minlength">
  <script src="/resources/testharness.js"></script>
  <script src="/resources/testharnessreport.js"></script>
</head>

<body>

  <textarea id="none"></textarea>
  <textarea id="negative" minlength=-5></textarea>
  <textarea id="non-numeric" minlength="not-a-number"></textarea>
  <textarea id="assign-negative"></textarea>
  <textarea id="assign-non-numeric"></textarea>

  <script>
    test(
      function () {
        assert_equals(document.getElementById("none").minLength, -1);
      }, "Unset minlength is -1");

    test(
      function () {
        assert_equals(document.getElementById("negative").minLength, -1);
      }, "Negative minlength is always -1");

    test(
      function () {
        assert_equals(document.getElementById("non-numeric").minLength, -1);
      }, "Non-numeric minlength is -1");

    test(
      function () {
        assert_throws_dom("INDEX_SIZE_ERR", function () {
          document.getElementById("assign-negative").minLength = -5;
        });
      }, "Assigning negative integer throws IndexSizeError");

    test(
      function () {
        document.getElementById("assign-non-numeric").minLength = "not-a-number";
        assert_equals(document.getElementById("assign-non-numeric").minLength, 0);
      }, "Assigning non-numeric to minlength sets minlength to 0");
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
  "source_name": "html/semantics/forms/the-textarea-element/textarea-minlength.html"
}
```
