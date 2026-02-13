# html/semantics/interestfor/interestfor-pseudo-element.tentative.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/interestfor/interestfor-pseudo-element.tentative.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8" />
<link rel="author" href="mailto:masonf@chromium.org">
<link rel="help" href="https://open-ui.org/components/interest-invokers.explainer">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-actions.js"></script>
<script src="/resources/testdriver-vendor.js"></script>
<script src="resources/invoker-utils.js"></script>

<div id=examples>
  <button interestfor=target>Button</button>
  <a href=# interestfor=target>Link</a>
</div>

<div id=target popover>Target</div>
<button id=unrelated>Unrelated</button>

<style>
  [interestfor] {
    interest-delay: 1000s;
  }
  ::interest-hint {
    content:"pseudo";
    display: block;
    position: absolute;
    width: 50px;
    height: 50px;
    left: 150px;
    padding:0;
    margin:0;
  }
  button, a {
    display:block;
    width: 100px;
    height: 50px;
  }
</style>

<script>
const target = document.getElementById('target');
const unrelated = document.getElementById('unrelated');

let interestFired = 0;
let loseInterestFired = 0;
target.addEventListener('interest', (e) => {
  ++interestFired;
});
target.addEventListener('loseinterest', (e) => {
  ++loseInterestFired;
});

function test(invoker) {
  promise_test(async (t) => {
    interestFired = 0, loseInterestFired = 0;
    await hoverOver(invoker);
    assert_equals(interestFired, 0, 'Hovering should not immediately show interest (delay)');
    await hoverOver(unrelated);
    assert_equals(interestFired, 0, 'No extra events');
    assert_equals(loseInterestFired, 0, 'No extra events');

    let rect = invoker.getBoundingClientRect();
    const buttonlocX = rect.x + 175;
    const buttonlocY = rect.y + 30;
    await new test_driver.Actions()
      .pointerMove(buttonlocX, buttonlocY, {})
      .send();
    assert_equals(interestFired, 0, 'Hovering the pseudo button should not show interest (delay)');
    await new test_driver.Actions()
      .addPointer("touchPointer", "touch")
      .pointerMove(buttonlocX, buttonlocY, {})
      .pointerDown()
      .pointerUp()
      .send();
    assert_equals(interestFired, 1, 'Clicking/tapping the pseudo button should show interest immediately');
    await new test_driver.Actions()
      .addPointer("touchPointer", "touch")
      .pointerMove(0,0,{origin: unrelated})
      .send();
    assert_equals(interestFired, 1, 'No extra events');
    assert_equals(loseInterestFired, 0, 'No "hover" with touch pointers, so we should\'t lose interest yet');
    await new test_driver.Actions()
      .addPointer("touchPointer", "touch")
      .pointerMove(0,0,{origin: unrelated})
      .pointerDown()
      .pointerUp()
      .send();
    await waitForRender();
    assert_equals(interestFired, 1, 'No extra events');
    assert_equals(loseInterestFired, 1, 'Tapping outside should lose interest');
  },`pseudo element should work for ${invoker.textContent}`);
}

document.querySelectorAll('#examples > *').forEach(test);
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
        "byte_end": 694,
        "byte_start": 687,
        "col": 1,
        "line": 20
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
  "source_name": "html/semantics/interestfor/interestfor-pseudo-element.tentative.html"
}
```
