# html/semantics/interactive-elements/the-dialog-element/dialog-light-dismiss-touch.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/interactive-elements/the-dialog-element/dialog-light-dismiss-touch.html",
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

<div id=fullscreen></div>
<dialog closedby=any>
  dialog
</dialog>

<style>
#fullscreen {
  position: absolute;
  inset: 0;
  z-index: 1;
}
</style>

<script>
promise_test(async () => {
  const fullscreen = document.getElementById('fullscreen');
  const dialog = document.querySelector('dialog');

  let fullscreenClicked = false;
  fullscreen.addEventListener('click', () => {
    fullscreenClicked = true;
  });
  dialog.showModal();

  await (new test_driver.Actions()
    .addPointer('finger', 'touch')
    .pointerMove(1, 1)
    .pointerDown()
    .pointerUp())
    .send();

  assert_false(dialog.open, 'Dialog should be closed by light dismiss.');
  assert_false(fullscreenClicked, 'Elements outside of the dialog should not receive a click.');
}, 'Dialog light dismiss should work with touch and not trigger a click event.');
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.style.not_allowed_here",
      "message": "Element “style” not allowed as child of “body” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 551,
        "byte_start": 544,
        "col": 1,
        "line": 16
      }
    },
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
  "source_name": "html/semantics/interactive-elements/the-dialog-element/dialog-light-dismiss-touch.html"
}
```
