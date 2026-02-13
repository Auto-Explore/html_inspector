# html/semantics/interactive-elements/the-dialog-element/dialog-open-2.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/interactive-elements/the-dialog-element/dialog-open-2.html",
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
<link rel=help href="https://bugs.webkit.org/show_bug.cgi?id=90931">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<dialog id=mydialog>It's my dialog.</dialog>

<script>
test(() => {
  const dialog = document.getElementById('mydialog');
  let computedStyle = window.getComputedStyle(dialog, null);
  assert_equals(computedStyle.getPropertyValue('display'), 'none');

  dialog.show();
  computedStyle = window.getComputedStyle(dialog, null);
  assert_equals(computedStyle.getPropertyValue('display'), 'block');

  dialog.close();
  computedStyle = window.getComputedStyle(dialog, null);

  assert_equals(computedStyle.getPropertyValue('display'), 'none');
  dialog.close();
}, "Tests that dialog is visible after show() is called and not visible after close() is called.");
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
  "source_name": "html/semantics/interactive-elements/the-dialog-element/dialog-open-2.html"
}
```
