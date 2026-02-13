# html/semantics/interactive-elements/the-dialog-element/top-layer-position.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/interactive-elements/the-dialog-element/top-layer-position.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel=author href="mailto:jarhar@chromium.org">
<link rel=help href="https://html.spec.whatwg.org/multipage/interactive-elements.html#the-dialog-element">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<body>
<script>
test(() => {
  const dialog = document.createElement('dialog');
  document.body.appendChild(dialog);

  dialog.style = 'position:static';
  assert_equals(getComputedStyle(dialog).position, 'static');
  dialog.showModal();
  assert_true(dialog.open);
  assert_equals(getComputedStyle(dialog).position, 'absolute',
    `dialog should be position:absolute when element.style has position:static.`);
  dialog.close();
  assert_false(dialog.open);

  dialog.style = 'position:relative';
  assert_equals(getComputedStyle(dialog).position, 'relative');
  dialog.showModal();
  assert_true(dialog.open);
  assert_equals(getComputedStyle(dialog).position, 'absolute',
    `dialog should be position:absolute when element.style has position:relative.`);
  dialog.close();
  assert_false(dialog.open);
}, `Verifies that position:static and position:relative computed to position:absolute in the top layer.`);
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
  "source_name": "html/semantics/interactive-elements/the-dialog-element/top-layer-position.html"
}
```
