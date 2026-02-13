# html/semantics/interactive-elements/the-dialog-element/popover-dialog-does-not-block-mouse-events.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/interactive-elements/the-dialog-element/popover-dialog-does-not-block-mouse-events.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>Popover dialogs should not block mouse events</title>
<link rel="author" title="Tim Nguyen" href="https://github.com/nt1m">
<link rel="help" href="https://html.spec.whatwg.org/multipage/interactive-elements.html#the-dialog-element">
<style>
    #div {
        height: 100px;
        width: 100px;
        background: red;
    }
</style>
<div id="div"></div>
<dialog id="dialog" popover="manual"></dialog>

<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-actions.js"></script>
<script src="/resources/testdriver-vendor.js"></script>
<script>
promise_test(async () => {
  const dialog = document.getElementById("dialog");
  dialog.showPopover();

  const div = document.getElementById("div");
  div.addEventListener("click", function(event) {
    div.firedOn = true;
    div.style.backgroundColor = "green";
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
  assert_true(div.firedOn, "div should have gotten a click event.");
}, "Ensure that popover dialogs do not block mouse events. To test manually, click the red box. The test succeeds if the red box turns green.");
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
  "source_name": "html/semantics/interactive-elements/the-dialog-element/popover-dialog-does-not-block-mouse-events.html"
}
```
