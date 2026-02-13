# html/semantics/interactive-elements/the-dialog-element/dialog-closedby-bounds-clicking.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/interactive-elements/the-dialog-element/dialog-closedby-bounds-clicking.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<meta name="timeout" content="long">
<link rel="author" href="mailto:wpt@keithcirkel.co.uk">
<link rel=help href="https://html.spec.whatwg.org/multipage/interactive-elements.html#dialog-light-dismiss">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-actions.js"></script>
<script src="/resources/testdriver-vendor.js"></script>

<dialog id="dialog">
  <div id="box"></div>
</dialog>

<style>
  dialog { inset: 0 auto auto 0; width: 100px; height: 100px; overflow: visible; }
  #box { height:200px; width:200px; margin:20px; background:blue; }
</style>

<script>
for (const method of ['show', 'showModal']) {
  promise_test(async (t) => {
    dialog.setAttribute('closedby','any');
    dialog[method]();
    const actions = new test_driver.Actions();
    await actions.pointerMove(1, 1, {origin:"viewport"})
      .pointerDown()
      .pointerUp()
      .send();
    assert_true(dialog.open, "Dialog should be open after clicking 1px-1px");
  },`Dialog ${method} will not light dismiss if clicked inside of the dialog bounds`);

  promise_test(async (t) => {
    dialog.setAttribute('closedby','any');
    dialog[method]();
    const actions = new test_driver.Actions();
    await actions.pointerMove(99, 99, {origin:"viewport"})
      .pointerDown()
      .pointerUp()
      .send();
    assert_true(dialog.open, "Dialog should be open after clicking 99px-99px");
  },`Dialog ${method} will not light dismiss if clicked inside of the dialog bounds (bottom right)`);

  promise_test(async (t) => {
    dialog.setAttribute('closedby','any');
    dialog[method]();
    const actions = new test_driver.Actions();
    await actions.pointerMove(150, 150, {origin:"viewport"})
      .pointerDown()
      .pointerUp()
      .send();
    assert_true(dialog.open, "Dialog should be open after clicking 150px-150px");
  },`Dialog ${method} will not light dismiss if clicked inside the overflowing div bounds (center)`);

  promise_test(async (t) => {
    dialog.setAttribute('closedby','any');
    dialog[method]();
    const actions = new test_driver.Actions();
    await actions.pointerMove(199, 199, {origin:"viewport"})
      .pointerDown()
      .pointerUp()
      .send();
    assert_true(dialog.open, "Dialog should be open after clicking 199px-199px");
  },`Dialog ${method} will not light dismiss if clicked inside the overflowing div bounds (bottom right)`);

  promise_test(async (t) => {
    dialog.setAttribute('closedby','any');
    dialog[method]();
    const actions = new test_driver.Actions();
    await actions.pointerMove(250, 250, {origin:"viewport"})
      .pointerDown()
      .pointerUp()
      .send();
    assert_false(dialog.open, "Dialog should be closed after clicking 250px-250px");
  },`Dialog ${method} light dismisses when clicked outside of the bounds of both the dialog and the div`);

  promise_test(async (t) => {
    dialog.setAttribute('closedby','any');
    dialog[method]();
    const actions = new test_driver.Actions();
    await actions.pointerMove(150, 1, {origin:"viewport"})
      .pointerDown()
      .pointerUp()
      .send();
    assert_false(dialog.open, "Dialog should be closed after clicking 150px-1px");
  },`Dialog ${method} light dismisses when clicked outside of the bounds of the dialog - where the Y direction is in-line with the div still`);

  promise_test(async (t) => {
    dialog.setAttribute('closedby','any');
    dialog[method]();
    const actions = new test_driver.Actions();
    await actions.pointerMove(1, 150, {origin:"viewport"})
      .pointerDown()
      .pointerUp()
      .send();
    assert_false(dialog.open, "Dialog should be closed after clicking 1px-150px");
  },`Dialog ${method} light dismisses when clicked outside of the bounds of the dialog - where the X direction is in-line with the div still`);
}
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
        "byte_end": 572,
        "byte_start": 565,
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
  "source_name": "html/semantics/interactive-elements/the-dialog-element/dialog-closedby-bounds-clicking.html"
}
```
