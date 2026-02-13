# html/semantics/popovers/popover-invoking-attribute.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/popovers/popover-invoking-attribute.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8" />
<title>Popover invoking attribute</title>
<link rel="author" href="mailto:masonf@chromium.org">
<link rel=help href="https://open-ui.org/components/popover.research.explainer">
<link rel=help href="https://html.spec.whatwg.org/multipage/popover.html">
<meta name="timeout" content="long">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-actions.js"></script>
<script src="/resources/testdriver-vendor.js"></script>
<script src="resources/popover-utils.js"></script>
<script src="resources/popover-invoking-attribute.js"></script>

<body>
<script>
runPopoverInvokerTests(["auto","manual"]);
</script>

<button popovertarget=p1>Toggle Popover 1</button>
<div popover id=p1 style="border: 5px solid red;top: 100px;left: 100px;">This is popover #1</div>

<script>
function clickOn(element) {
  const actions = new test_driver.Actions();
  return actions.pointerMove(0, 0, {origin: element})
    .pointerDown({button: actions.ButtonType.LEFT})
    .pointerUp({button: actions.ButtonType.LEFT})
    .send();
}

const popover = document.querySelector('[popover]');
const button = document.querySelector('button');
let showCount = 0;
let hideCount = 0;
popover.addEventListener('beforetoggle',(e) => {
  if (e.newState === "open")
      ++showCount;
    else
      ++hideCount;
  });

async function assertState(expectOpen,expectShow,expectHide) {
  assert_equals(popover.matches(':popover-open'),expectOpen,'Popover open state is incorrect');
  await new Promise(resolve => requestAnimationFrame(resolve));
  assert_equals(showCount,expectShow,'Show count is incorrect');
  assert_equals(hideCount,expectHide,'Hide count is incorrect');
}

window.addEventListener('load', () => {
  promise_test(async () => {
    showCount = hideCount = 0;
    await assertState(false,0,0);
    await clickOn(button);
    await assertState(true,1,0);
    popover.hidePopover();
    await assertState(false,1,1);
    button.click();
    await assertState(true,2,1);
    popover.hidePopover();
    await assertState(false,2,2);
  }, "Clicking a popovertarget button opens a closed popover (also check event counts)");

  promise_test(async () => {
    showCount = hideCount = 0;
    await assertState(false,0,0);
    await clickOn(button);
    await assertState(true,1,0);
    await clickOn(button);
    await assertState(false,1,1);
  }, "Clicking a popovertarget button closes an open popover (also check event counts)");
});
</script>
```

```json
{
  "messages": [
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/semantics/popovers/popover-invoking-attribute.html"
}
```
