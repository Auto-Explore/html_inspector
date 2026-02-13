# html/semantics/interactive-elements/the-dialog-element/submit-dialog-close-event.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/interactive-elements/the-dialog-element/submit-dialog-close-event.html",
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
<link rel=help href="https://bugs.chromium.org/p/chromium/issues/detail?id=304827">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<dialog>
  <form method="dialog">
    <input id="goodbye" type="submit" value="Goodbye">
    <input id="hello" type="submit" value="Hello">
  </form>
</dialog>

<script>
async_test(t => {
  const dialog = document.querySelector('dialog');
  dialog.show();
  dialog.addEventListener('close', t.step_func(() => {
    assert_false(dialog.open);
    assert_equals(dialog.returnValue, 'Goodbye');

    dialog.show();
    dialog.addEventListener('close', t.step_func_done(() => {
      assert_false(dialog.open);
      assert_equals(dialog.returnValue, 'Hello');
    }));
    document.querySelector('#hello').click();
  }), {once: true});

  document.querySelector('#goodbye').click();
}, 'Tests submitting a dialog on a close event triggered by a previous submission.');
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
  "source_name": "html/semantics/interactive-elements/the-dialog-element/submit-dialog-close-event.html"
}
```
