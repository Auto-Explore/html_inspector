# html/semantics/interactive-elements/the-dialog-element/closed-dialog-does-not-block-mouse-events.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/interactive-elements/the-dialog-element/closed-dialog-does-not-block-mouse-events.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel=author href="mailto:jarhar@chromium.org">
<link rel=help href="https://html.spec.whatwg.org/multipage/interactive-elements.html#the-dialog-element">
<link rel=help href="https://bugs.webkit.org/show_bug.cgi?id=110952">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-actions.js"></script>
<script src="/resources/testdriver-vendor.js"></script>

<style>
#div {
    height: 100px;
    width: 100px;
    background: red;
}
</style>
<div id=div></div>
<dialog id="dialog"></dialog>
<dialog></dialog>

<script>
promise_test(async () => {
  const dialog = document.getElementById('dialog');
  dialog.showModal();
  dialog.close();

  const div = document.getElementById('div');
  div.addEventListener('click', function(event) {
    div.firedOn = true;
    div.style.backgroundColor = 'green';
  });

  var absoluteTop = 0;
  var absoluteLeft = 0;
  for (var parentNode = div; parentNode; parentNode = parentNode.offsetParent) {
    absoluteLeft += parentNode.offsetLeft;
    absoluteTop += parentNode.offsetTop;
  }

  const x = absoluteLeft + div.offsetWidth / 2;
  const y = absoluteTop + div.offsetHeight / 2;
  const actions = new test_driver.Actions()
    .pointerMove(x, y)
    .pointerDown()
    .pointerUp()
    .pointerMove(0, 0);
  await actions.send();
  assert_true(div.firedOn, 'div should have gotten a click event.');
}, 'Ensure that closed dialogs do not block mouse events. To test manually, click the red box. The test succeeds if the red box turns green.');
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
  "source_name": "html/semantics/interactive-elements/the-dialog-element/closed-dialog-does-not-block-mouse-events.html"
}
```
