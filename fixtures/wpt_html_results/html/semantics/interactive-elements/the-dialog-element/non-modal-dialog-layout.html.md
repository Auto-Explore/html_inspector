# html/semantics/interactive-elements/the-dialog-element/non-modal-dialog-layout.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/interactive-elements/the-dialog-element/non-modal-dialog-layout.html",
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
<link rel=help href="https://bugs.chromium.org/p/chromium/issues/detail?id=382594">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<style>
/* Remove body margin and dialog styles for easier positioning expected values */
body {
  height: 10000px;
  margin: 0;
}

dialog {
  margin: 0;
  border: 0;
  padding: 0;
  width: auto;
  height: auto;
  max-width: initial;
  max-height: initial;
}

#absolute-div {
  position: absolute;
  top: 800px;
  height: 50px;
  width: 90%;
}

#relative-div {
  position: relative;
  top: 20px;
  height: 30px;
}
</style>

<div id="absolute-div">
  <div id="relative-div">
    <dialog id="dialog">It is my dialog.</dialog>
  </div>
</div>

<script>
test(() => {
  const dialog = document.querySelector('#dialog');
  const div = document.querySelector('#div-dialog');
  const relativeContainer = document.querySelector('#relative-div');
  const offset = 50;
  dialog.style.top = offset + 'px';
  dialog.style.left = offset + 'px';

  dialog.style.position = 'absolute';
  dialog.show();
  assert_equals(
    dialog.getBoundingClientRect().top,
    relativeContainer.getBoundingClientRect().top + offset,
    'Absolute position.');
  assert_equals(
    dialog.getBoundingClientRect().left,
    relativeContainer.getBoundingClientRect().left + offset,
    'Absolute position.');

  dialog.style.position = 'static';
  assert_true(dialog.open);
  assert_equals(
    dialog.getBoundingClientRect().top,
    relativeContainer.getBoundingClientRect().top,
    'Static position.');
  assert_equals(
    dialog.getBoundingClientRect().left,
    relativeContainer.getBoundingClientRect().left,
    'Static position.');
  dialog.close();

  dialog.style.position = 'relative';
  dialog.show();
  assert_equals(
    dialog.getBoundingClientRect().top,
    relativeContainer.getBoundingClientRect().top + offset,
    'Relative position.');
  assert_equals(
    dialog.getBoundingClientRect().left,
    relativeContainer.getBoundingClientRect().left + offset,
    'Relative position.');
  dialog.close();

  dialog.style.position = 'fixed';
  dialog.show();
  assert_equals(
    dialog.getBoundingClientRect().top,
    offset,
    'Fixed position.');
  assert_equals(
    dialog.getBoundingClientRect().left,
    offset,
    'Fixed position.');
  dialog.close();
}, 'Tests layout of non-modal dialogs.');
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
  "source_name": "html/semantics/interactive-elements/the-dialog-element/non-modal-dialog-layout.html"
}
```
