# html/semantics/interactive-elements/the-dialog-element/inert-svg-hittest.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/interactive-elements/the-dialog-element/inert-svg-hittest.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>Hit-testing with SVG made inert by modal dialog</title>
<link rel="author" title="Tim Nguyen" href="https://github.com/nt1m">
<link rel="help" href="https://html.spec.whatwg.org/multipage/interaction.html#inert">
<meta assert="assert" content="SVG made inert by modal dialog should be unreachable with hit-testing">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-vendor.js"></script>
<script src="/resources/testdriver-actions.js"></script>

<div id="wrapper">
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 500 500">
        <rect width="500" height="500" id="target" fill="red">
    </svg>
</div>

<dialog id="dialog">Content behind the open modal dialog should not be clickable</dialog>

<style>
dialog::backdrop {
    display: none;
}
</style>

<script>
const dialog = document.getElementById("dialog");
const wrapper = document.getElementById("wrapper");
const target = document.getElementById("target");

promise_test(async function() {
    dialog.showModal();
    this.add_cleanup(() => dialog.close());

    let reachedTarget = false;
    target.addEventListener("mousedown", () => {
        reachedTarget = true;
    }, { once: true });

    let wrapperRect = wrapper.getBoundingClientRect();
    await new test_driver.Actions()
        .pointerMove(wrapperRect.x + 1, wrapperRect.y + 1, { origin: "viewport" })
        .pointerDown()
        .send();

    assert_false(target.matches(":active"), "target is not active");
    assert_false(target.matches(":hover"), "target is not hovered");
    assert_false(reachedTarget, "target didn't get event");
}, "Hit-testing doesn't reach contents of an inert SVG");

promise_test(async function() {
    assert_false(dialog.open, "dialog is closed");

    let reachedTarget = false;
    target.addEventListener("mousedown", () => {
        reachedTarget = true;
    }, { once: true });

    await new test_driver.Actions()
        .pointerMove(0, 0, { origin: wrapper })
        .pointerDown()
        .send();

    assert_true(target.matches(":active"), "target is active");
    assert_true(reachedTarget, "target got event");
}, "Hit-testing can reach contents of a no longer inert SVG");
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
        "byte_end": 897,
        "byte_start": 890,
        "col": 1,
        "line": 21
      }
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/semantics/interactive-elements/the-dialog-element/inert-svg-hittest.html"
}
```
