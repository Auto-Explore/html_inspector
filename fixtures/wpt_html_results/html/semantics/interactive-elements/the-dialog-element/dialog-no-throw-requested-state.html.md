# html/semantics/interactive-elements/the-dialog-element/dialog-no-throw-requested-state.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/interactive-elements/the-dialog-element/dialog-no-throw-requested-state.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel=author href="mailto:jarhar@chromium.org">
<link rel=help href="https://github.com/whatwg/html/pull/9142">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<dialog>hello</dialog>

<script>
test(() => {
  const dialog = document.querySelector('dialog');

  // calling close() on a dialog that is already closed should not throw.
  dialog.close();

  dialog.show();
  // calling show() on a dialog that is already showing non-modal should not throw.
  dialog.show();
  assert_throws_dom('InvalidStateError', () => dialog.showModal(),
    'Calling showModal() on a dialog that is already showing non-modal should throw.');
  dialog.close();

  dialog.showModal();
  assert_throws_dom('InvalidStateError', () => dialog.show(),
    'Calling show() on a dialog that is already showing modal should throw.');
  // calling showModal() on a dialog that is already showing modal should not throw.
  dialog.showModal();
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
  "source_name": "html/semantics/interactive-elements/the-dialog-element/dialog-no-throw-requested-state.html"
}
```
