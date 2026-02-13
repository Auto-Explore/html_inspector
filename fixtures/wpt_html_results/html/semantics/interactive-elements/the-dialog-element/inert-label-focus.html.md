# html/semantics/interactive-elements/the-dialog-element/inert-label-focus.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/interactive-elements/the-dialog-element/inert-label-focus.html",
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
<link rel=help href="https://bugs.chromium.org/p/chromium/issues/detail?id=242848">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-actions.js"></script>
<script src="/resources/testdriver-vendor.js"></script>

<label for="submit">Label for Submit</label>
<dialog>
  <input id="text" type="text">
  <input id="submit" type="submit">
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

  document.querySelector('dialog').showModal();
  document.querySelector('#text').focus();

  label = document.querySelector('label');
  submit = document.querySelector('#submit');
  label.focus();
  assert_equals(document.activeElement, submit,
    'label.focus() should send focus to the target.');

  await clickOn(label);
  assert_not_equals(document.activeElement, label,
    'Clicking the label should not focus the label.');
  assert_not_equals(document.activeElement, submit,
    'Clicking the label should not focus the submit input.');
}, 'Tests focusing of an inert label for a non-inert target.');
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
  "source_name": "html/semantics/interactive-elements/the-dialog-element/inert-label-focus.html"
}
```
