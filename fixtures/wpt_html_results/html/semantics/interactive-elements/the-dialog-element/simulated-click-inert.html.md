# html/semantics/interactive-elements/the-dialog-element/simulated-click-inert.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/interactive-elements/the-dialog-element/simulated-click-inert.html",
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
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-actions.js"></script>
<script src="/resources/testdriver-vendor.js"></script>

<p>Ensure that simulated click is still dispatched to an inert node.
To test manually, click the CLICK ME label and verify it does change the value of the checkbox.</p>
<div>
</div>
<input type="checkbox" id="target">
<dialog><label for="target">CLICK ME</label></dialog>

<script>
promise_test(async () => {
  async function clickOn(element) {
    const actions = new test_driver.Actions()
      .pointerMove(0, 0, {origin: element})
      .pointerDown()
      .pointerUp()
    await actions.send();
  }

  document.querySelector('dialog').showModal();
  await clickOn(document.querySelector('label'));
  assert_true(document.getElementById('target').checked);
}, 'Ensure that simulated click is still dispatched to an inert node.');
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
  "source_name": "html/semantics/interactive-elements/the-dialog-element/simulated-click-inert.html"
}
```
