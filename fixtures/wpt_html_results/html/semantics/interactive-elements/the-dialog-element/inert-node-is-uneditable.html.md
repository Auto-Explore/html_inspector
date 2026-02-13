# html/semantics/interactive-elements/the-dialog-element/inert-node-is-uneditable.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/interactive-elements/the-dialog-element/inert-node-is-uneditable.html",
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
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-actions.js"></script>
<script src="/resources/testdriver-vendor.js"></script>

<span id="not-editable" contenteditable>I'm not editable while the dialog is showing.</span>
<dialog>
  <span id="editable" contenteditable>I'm editable.</span>
</dialog>

<script>
promise_test(async () => {
  async function clickOn(element) {
    let absoluteTop = 0;
    let absoluteLeft = 0;
    for (let parentNode = element; parentNode; parentNode = parentNode.offsetParent) {
      absoluteLeft += parentNode.offsetLeft;
      absoluteTop += parentNode.offsetTop;
    }

    const x = Math.round(absoluteLeft + element.offsetWidth / 2);
    const y = Math.round(absoluteTop + element.offsetHeight / 2);
    const actions = new test_driver.Actions()
      .pointerMove(x, y)
      .pointerDown()
      .pointerUp()
      .pointerMove(0, 0);
    await actions.send();
  }

  dialog = document.querySelector('dialog');
  dialog.showModal();
  notEditable = document.querySelector('#not-editable');
  editable = document.querySelector('#editable');

  await clickOn(notEditable);
  oldValue = notEditable.textContent;
  await (new test_driver.Actions().keyDown('a').keyUp('a').send());
  assert_equals(notEditable.textContent, oldValue);

  await clickOn(editable);
  oldValue = editable.textContent;
  await (new test_driver.Actions().keyDown('a').keyUp('a').send());
  assert_not_equals(editable.textContent, oldValue);

  notEditable.remove();
  editable.remove();
}, 'Test that inert nodes cannot be edited. The test passes if the only text you can edit is in the dialog.');
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
  "source_name": "html/semantics/interactive-elements/the-dialog-element/inert-node-is-uneditable.html"
}
```
