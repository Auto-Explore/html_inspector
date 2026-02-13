# html/semantics/interactive-elements/the-dialog-element/dialog-keydown-preventDefault.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/interactive-elements/the-dialog-element/dialog-keydown-preventDefault.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
<head>
  <title>Test cancel event with preventDefault on keydown event for dialog element</title>
  <script src="/resources/testharness.js"></script>
  <script src="/resources/testharnessreport.js"></script>
  <script src="/resources/testdriver.js"></script>
  <script src="/resources/testdriver-vendor.js"></script>
  <link rel="help" href="https://bugs.webkit.org/show_bug.cgi?id=227534">
  <link rel="help" href="https://bugzilla.mozilla.org/show_bug.cgi?id=1322947">
</head>
<body>
<p>Test cancel event with preventDefault on keydown event for dialog element</p>
<dialog>
  <p>Hello World</p>
</dialog>
<script>
  setup({ single_test: true });

  var hasCancelEventFired = false;

  const dialog = document.querySelector("dialog");

  const verify = () => {
    assert_false(hasCancelEventFired, "cancel should not be fired");
    assert_true(hasKeydownEventFired, "document level keydown event should be fired");
    done();
  };

  dialog.addEventListener("cancel", function(event) {
    hasCancelEventFired = true;
  });

  document.addEventListener("keydown", function(event) {
    hasKeydownEventFired = true;
    event.preventDefault();
    step_timeout(function() {
      verify();
    }, 0);
  });
  dialog.showModal();
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
  "source_name": "html/semantics/interactive-elements/the-dialog-element/dialog-keydown-preventDefault.html"
}
```
