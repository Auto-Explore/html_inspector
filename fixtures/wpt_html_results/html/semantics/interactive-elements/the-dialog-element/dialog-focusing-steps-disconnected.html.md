# html/semantics/interactive-elements/the-dialog-element/dialog-focusing-steps-disconnected.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/interactive-elements/the-dialog-element/dialog-focusing-steps-disconnected.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
<head>
<title>Test focusing steps when dialog is disconnected</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
</head>
<body>
<input>
<script>
test(function() {
  const outerInput = document.querySelector("input");
  outerInput.focus();
  assert_equals(document.activeElement, outerInput,
    "Focus should be on element we just focused");

  const dialog = document.createElement("dialog");
  assert_false(dialog.open, "Dialog should initially be closed");
  assert_false(dialog.hasAttribute('open'), "Dialog should initially be closed");

  const innerInput = document.createElement("input");
  innerInput.autofocus = true;
  dialog.append(innerInput);

  dialog.show();
  this.add_cleanup(() => { dialog.close(); });
  assert_true(dialog.open, "Disconnected dialog can still be open");


  assert_equals(document.activeElement, outerInput, "Focusing steps should not change focus");
}, "dialog.show(): focusing steps should not change focus on disconnected <dialog>");

test(function() {
  assert_throws_dom("InvalidStateError", () => {
    document.createElement("dialog").showModal();
  });
}, "dialog.showModal() should throw on disconnected <dialog>");
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
  "source_name": "html/semantics/interactive-elements/the-dialog-element/dialog-focusing-steps-disconnected.html"
}
```
