# html/semantics/interactive-elements/the-dialog-element/synthetic-click-inert.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/interactive-elements/the-dialog-element/synthetic-click-inert.html",
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
<link rel=help href="https://bugs.chromium.org/p/chromium/issues/detail?id=241699">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<style>
dialog {
  width: 50px;
}
</style>

<button>Click me</button>
<div id="div">Click me too</div>
<dialog></dialog>

<script>
test(() => {
  dialog = document.querySelector('dialog');
  dialog.showModal();

  const button = document.querySelector('button');
  const div = document.getElementById('div');
  let clicked = false;

  [button, div].forEach(function(element) {
    element.addEventListener('click', () => clicked = true);

    clicked = false;
    element.click();
    assert_true(clicked, 'Calling click() on ' + element.tagName);

    clicked = false;
    element.dispatchEvent(new Event('click'));
    assert_true(clicked, 'Calling dispatchEvent() on ' + element.tagName);
  });
}, 'Test that inert nodes still get programmatic click events');
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
  "source_name": "html/semantics/interactive-elements/the-dialog-element/synthetic-click-inert.html"
}
```
