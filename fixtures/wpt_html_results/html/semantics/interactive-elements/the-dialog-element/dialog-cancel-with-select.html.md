# html/semantics/interactive-elements/the-dialog-element/dialog-cancel-with-select.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/interactive-elements/the-dialog-element/dialog-cancel-with-select.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
<head>
  <title>Test dialog modal is closed by escape key with select focused</title>
  <script src="/resources/testharness.js"></script>
  <script src="/resources/testharnessreport.js"></script>
  <script src="/resources/testdriver.js"></script>
  <script src="/resources/testdriver-vendor.js"></script>
  <link rel="help" href="https://bugs.webkit.org/show_bug.cgi?id=227534">
  <link rel="help" href="https://bugzilla.mozilla.org/show_bug.cgi?id=1322947">
</head>
<body>
<p>Test dialog modal is closed by escape key with select focused</p>
<dialog id="dialog">
  <select>
    <option value="one">one</option>
    <option value="two">two</option>
  </select>
</dialog>

<script>
  setup({ single_test: true });

  const dialog = document.getElementById("dialog");
  const select = document.querySelector("select");

  dialog.addEventListener("close", function() {
    assert_false(dialog.open, "dialog with select is closed");
    done();
  });
  dialog.showModal();
  assert_true(select == document.activeElement, "select element should be focused");

  test_driver.send_keys(document.documentElement, "\uE00C"); // ESC key
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
  "source_name": "html/semantics/interactive-elements/the-dialog-element/dialog-cancel-with-select.html"
}
```
