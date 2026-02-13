# html/semantics/interactive-elements/the-dialog-element/dialog-light-dismiss-pointer-capture.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/interactive-elements/the-dialog-element/dialog-light-dismiss-pointer-capture.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel=author href="mailto:jarhar@chromium.org">
<link rel=help href="https://github.com/w3c/pointerevents/issues/542">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-vendor.js"></script>
<script src="/resources/testdriver-actions.js"></script>

<dialog id=dialog closedby=any>
  dialog
  <button>pointer capturing button</button>
</dialog>

<script>
const dialog = document.getElementById('dialog');

function click() {
  return (new test_driver.Actions()
    .pointerMove(1, 1)
    .pointerDown()
    .pointerUp())
    .send();
}

function touch() {
  return (new test_driver.Actions()
    .addPointer('finger', 'touch')
    .pointerMove(1, 1)
    .pointerDown()
    .pointerUp())
    .send();
}

[true, false].forEach(useTouch => {
  promise_test(async t => {
    let usePointerCapture = true;
    t.add_cleanup(() => {
      usePointerCapture = false;
      dialog.close();
    });

    const button = dialog.querySelector('button');
    document.body.addEventListener('pointerdown', event => {
      if (usePointerCapture) {
        button.setPointerCapture(event.pointerId);
      }
    });

    dialog.showModal();
    assert_true(dialog.open, 'dialog should be open at the start of the test.');

    if (useTouch) {
      await touch();
    } else {
      await click();
    }
    assert_true(dialog.open, 'dialog should not light dismiss when pointer capture is being used.');

    usePointerCapture = false;
    if (useTouch) {
      await touch();
    } else {
      await click();
    }
    assert_false(dialog.open, 'dialog should light dismiss when pointer capture is not being used.');
  }, `Light dismiss on dialog with ${useTouch ? 'touch' : 'mouse'} input with pointer capture.`);
});
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
  "source_name": "html/semantics/interactive-elements/the-dialog-element/dialog-light-dismiss-pointer-capture.html"
}
```
