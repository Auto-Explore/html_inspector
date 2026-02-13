# html/semantics/interactive-elements/the-dialog-element/dialog-light-dismiss-drag.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/interactive-elements/the-dialog-element/dialog-light-dismiss-drag.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel=author href="mailto:jarhar@chromium.org">
<link rel=help href="https://issues.chromium.org/issues/425579196">
<link rel=help href="https://github.com/w3c/pointerevents/issues/542">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-actions.js"></script>
<script src="/resources/testdriver-vendor.js"></script>

<button id=outside>outside dialog</button>
<dialog closedby=any>dialog</dialog>

<script>
const dialog = document.querySelector('dialog');
const outside = document.getElementById('outside');

promise_test(async () => {
  dialog.showModal();
  assert_true(dialog.open, 'dialog should be open after showModal().');
  await (new test_driver.Actions()
    .pointerMove(0, 0, {origin: dialog})
    .pointerDown()
    .pointerMove(0, 0, {origin: outside})
    .pointerUp())
    .send();
  assert_true(dialog.open, 'dialog should still be open after clicking and dragging.');
}, 'Dialog should not light dismiss when clicking inside the dialog and dragging outside of it.');
</script>
```

```json
{
  "messages": [
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
  "source_name": "html/semantics/interactive-elements/the-dialog-element/dialog-light-dismiss-drag.html"
}
```
