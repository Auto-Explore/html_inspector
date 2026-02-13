# html/semantics/interactive-elements/the-dialog-element/dialog-close-event.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/interactive-elements/the-dialog-element/dialog-close-event.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel=author href="mailto:jarhar@chromium.org">
<link rel=author href="mailto:falken@chromium.org">
<link rel=help href="https://html.spec.whatwg.org/multipage/interactive-elements.html#the-dialog-element">
<link rel=help href="https://bugs.chromium.org/p/chromium/issues/detail?id=276785">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<dialog></dialog>

<script>
async_test(t => {
  document.addEventListener('close', t.step_func_done(() => {
    t.assert_unreached(`The 'close' event unexpectedly bubbled.`);
  }));

  closedCount = 0;
  dialog = document.querySelector('dialog');
  dialog.addEventListener('close', function(event) {
    const selfDialog = this;
    t.step(() => {
      closedCount++;
      assert_equals(selfDialog, dialog);
      assert_false(dialog.open);
      assert_false(event.cancelable);
      event.preventDefault();

      if (closedCount == 1) {
        dialog.show();
        dialog.close();
        assert_equals(closedCount, 1, `dialog's close event handler shouldn't be called synchronously.`);
      } else if (closedCount == 2) {
        t.done();
      }
    });
  });

  dialog.show();
  dialog.close();

  // Verify that preventDefault() didn't cancel closing.
  assert_false(dialog.open);

  // dialog's close event handler shouldn't be called synchronously.
  assert_equals(closedCount, 0);
}, "Test that dialog receives a close event upon closing.");
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
  "source_name": "html/semantics/interactive-elements/the-dialog-element/dialog-close-event.html"
}
```
