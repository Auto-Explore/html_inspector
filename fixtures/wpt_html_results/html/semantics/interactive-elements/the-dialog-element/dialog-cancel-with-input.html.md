# html/semantics/interactive-elements/the-dialog-element/dialog-cancel-with-input.html

Counts:
- errors: 1
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/interactive-elements/the-dialog-element/dialog-cancel-with-input.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
<head>
  <title>Test dialog modal is closed by escape key with input focused</title>
  <script src="/resources/testharness.js"></script>
  <script src="/resources/testharnessreport.js"></script>
  <script src="/resources/testdriver.js"></script>
  <script src="/resources/testdriver-vendor.js"></script>
  <link rel="help" href="https://bugs.webkit.org/show_bug.cgi?id=227534">
  <link rel="help" href="https://bugzilla.mozilla.org/show_bug.cgi?id=1322947">
</head>
<body>
<p>Test dialog modal is closed by escape key with input focused</p>
<dialog id="dialog">
  <p>Hello World</p>
</dialog>

<dialog id="dialogWithAutofocus">
  <input autofocus/>
</dialog>

<script>
  setup({ single_test: true });

  const triggerEscKey = () => {
    test_driver.send_keys(document.documentElement, "\uE00C"); // ESC key
  };

  /* Make sure we still cancel the dialog even if the input element is focused */
  function runTestCancelWhenInputFocused() {
    const dialog = document.getElementById("dialogWithAutofocus");
    const input = document.querySelector("input");

    dialog.addEventListener("close", function() {
      assert_false(dialog.open, "dialog with input autofocused is closed");
      done();
    });
    dialog.showModal();
    assert_true(input == document.activeElement, "input element should be focused");

    triggerEscKey();
  }

  const dialog = document.getElementById("dialog");

  dialog.addEventListener("close", function() {
    assert_false(dialog.open, "dialog closed");
    step_timeout(function() {
      runTestCancelWhenInputFocused();
    }, 0);
  });

  dialog.showModal();
  triggerEscKey();
</script>
</pre>
</body>
</html>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.parser.stray_end_tag",
      "message": "Stray end tag “pre”.",
      "severity": "Error",
      "span": {
        "byte_end": 1660,
        "byte_start": 1654,
        "col": 1,
        "line": 56
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
  "source_name": "html/semantics/interactive-elements/the-dialog-element/dialog-cancel-with-input.html"
}
```
