# html/semantics/interactive-elements/the-dialog-element/non-modal-canceling.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/interactive-elements/the-dialog-element/non-modal-canceling.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel=author href="mailto:masonf@chromium.org">
<link rel=help href="https://html.spec.whatwg.org/multipage/interactive-elements.html#the-dialog-element">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-actions.js"></script>
<script src="/resources/testdriver-vendor.js"></script>
<script src="/close-watcher/resources/helpers.js"></script>

<dialog>Dialog</dialog>

<script>
promise_test(async () => {
  const dialog = document.querySelector('dialog');
  assert_false(dialog.open);
  dialog.show();
  assert_true(dialog.open);
  await sendEscKey();
  assert_true(dialog.open,'Escape does not close a non-modal dialog');
  dialog.close();
  dialog.showModal();
  assert_true(dialog.open);
  await sendEscKey();
  assert_false(dialog.open,'Escape does close a modal dialog');
},'Non-modal dialogs should not be cancelable via ESC');
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
  "source_name": "html/semantics/interactive-elements/the-dialog-element/non-modal-canceling.html"
}
```
