# html/semantics/interactive-elements/the-dialog-element/inert-inlines.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/interactive-elements/the-dialog-element/inert-inlines.html",
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

<p>
To test manually, click on all the "Click me"s.
The test fails if you see red.
</p>

<style>
dialog {
  width: 50px;
}
</style>

<a id="a" href="javascript:void(0)">Click me</a>
<button id="button">Click me</button>
<div id="div" style="background-color: blue; width: 50px; height: 50px">Click meeee</div>
<span id="span">Click me</span>
<div id="dialog-parent" style="width: 50px; height: 50px">
  <span id="dialog-sibling">Click meeee</span>
  <dialog></dialog>
</div>

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

  function eventFiredOnInertElement(e) {
    e.target.style.background = 'red';
    inertElementFiredOn = true;
  }

  inertElements = ['a', 'button', 'div', 'span']
  inertElements.forEach(function(id) {
    element = document.getElementById(id);
    element.addEventListener('click', eventFiredOnInertElement);
    element.addEventListener('mousemove', eventFiredOnInertElement);
  });

  document.addEventListener('click', function(e) {
    document.firedOn = true;
  });

  document.getElementById('dialog-parent').addEventListener('click', function(e) {
    e.target.firedOn = true;
  });

  document.querySelector('dialog').showModal();
  for (const id of inertElements) {
    expectedTarget = document;
    if (id == 'dialog-sibling')
      expectedTarget = document.getElementById('dialog-parent')
    element = document.getElementById(id);
    inertElementFiredOn = false;
    expectedTarget.firedOn = false;
    await clickOn(element);
    assert_false(inertElementFiredOn, 'clicking on ' + id);
    assert_true(expectedTarget.firedOn, 'clicking on ' + id);
  }
}, 'Tests that inert inlines do not receive mouse events.');
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
        "byte_end": 676,
        "byte_start": 669,
        "col": 1,
        "line": 17
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
  "source_name": "html/semantics/interactive-elements/the-dialog-element/inert-inlines.html"
}
```
