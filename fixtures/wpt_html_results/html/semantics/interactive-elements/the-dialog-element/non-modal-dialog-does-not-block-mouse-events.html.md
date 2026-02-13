# html/semantics/interactive-elements/the-dialog-element/non-modal-dialog-does-not-block-mouse-events.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/interactive-elements/the-dialog-element/non-modal-dialog-does-not-block-mouse-events.html",
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
<link rel=help href="https://bugs.webkit.org/show_bug.cgi?id=110952">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-actions.js"></script>
<script src="/resources/testdriver-vendor.js"></script>

<p>
To test manually, click the red box. The test succeeds if the red box turns green.
</p>

<style>
#div {
    height: 100px;
    width: 100px;
    background: red;
}
</style>

<div id="div"></div>
<dialog id="dialog"></dialog>

<script>
promise_test(async () => {
  async function clickOn(element) {
    const actions = new test_driver.Actions()
      .pointerMove(0, 0, {origin: element})
      .pointerDown()
      .pointerUp()
      .pointerMove(0, 0);
    await actions.send();
  }

  const dialog = document.getElementById('dialog');
  dialog.show();

  const div = document.getElementById('div');
  div.firedOn = false;
  div.addEventListener('click', function(event) {
    div.firedOn = true;
    div.style.backgroundColor = 'green';
  });

  await clickOn(div);

  assert_true(div.firedOn);
}, 'Ensure that non-modal dialogs do not block mouse events.');
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.style.not_allowed_here",
      "message": "Element “style” not allowed as child of “body” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 666,
        "byte_start": 659,
        "col": 1,
        "line": 16
      }
    },
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
  "source_name": "html/semantics/interactive-elements/the-dialog-element/non-modal-dialog-does-not-block-mouse-events.html"
}
```
