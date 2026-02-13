# html/semantics/interactive-elements/the-dialog-element/dialog-focusing-steps-inert.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/interactive-elements/the-dialog-element/dialog-focusing-steps-inert.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
<head>
<title>Test focusing steps when dialog is inert</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
</head>
<body>
<input id="outer-input">
<dialog>
  <input autofocus>
</dialog>
<script>
function test_focusing_steps_with_inert_dialog(test, isModal) {
  const outerInput = document.querySelector("#outer-input");
  outerInput.focus();
  assert_equals(document.activeElement, outerInput,
    "Focus should be on element we just focused");

  const dialog = document.querySelector("dialog");
  assert_false(dialog.open, "Dialog should initially be closed");

  dialog.inert = true;
  test.add_cleanup(() => { dialog.inert = false; });

  if (isModal) {
    dialog.showModal();
    test.add_cleanup(() => { dialog.close(); });
    assert_equals(document.activeElement, document.body,
      "dialog.showModal(): focusing steps should apply focus fixup rule when dialog is inert");
  } else {
    dialog.show();
    test.add_cleanup(() => { dialog.close(); });
    assert_equals(document.activeElement, outerInput,
      "dialog.show(): focusing steps should not change focus when dialog is inert");
  }
}

test(function() {
  test_focusing_steps_with_inert_dialog(this, false);
}, "dialog.show(): focusing steps should not change focus when dialog is inert");

test(function() {
  test_focusing_steps_with_inert_dialog(this, true);
}, "dialog.showModal(): focusing steps should apply focus fixup rule when dialog is inert");
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
  "source_name": "html/semantics/interactive-elements/the-dialog-element/dialog-focusing-steps-inert.html"
}
```
