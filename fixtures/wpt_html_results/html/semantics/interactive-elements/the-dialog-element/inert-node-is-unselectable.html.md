# html/semantics/interactive-elements/the-dialog-element/inert-node-is-unselectable.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/interactive-elements/the-dialog-element/inert-node-is-unselectable.html",
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
<link rel=help href="https://bugs.chromium.org/p/chromium/issues/detail?id=252071">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

Here is a text node you can't select while the dialog is open.
<dialog>I'm selectable.</dialog>

<script>
test(() => {
  const dialog = document.querySelector('dialog');
  dialog.showModal();
  document.execCommand('SelectAll');
  assert_equals(window.getSelection().toString(), "I'm selectable.");
}, 'Test that inert nodes cannot be selected. The test passes if the only text you can select is inside the dialog.');
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
  "source_name": "html/semantics/interactive-elements/the-dialog-element/inert-node-is-unselectable.html"
}
```
