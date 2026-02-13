# html/semantics/interactive-elements/the-dialog-element/dialog-closedby-start-open.html

Counts:
- errors: 0
- warnings: 4
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/interactive-elements/the-dialog-element/dialog-closedby-start-open.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset="utf-8">
<meta name="timeout" content="long">
<link rel=help href="https://html.spec.whatwg.org/multipage/interactive-elements.html#dialog-light-dismiss">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-actions.js"></script>
<script src="/resources/testdriver-vendor.js"></script>
<script src="../../popovers/resources/popover-utils.js"></script>

<dialog id=test1 open closedby="any"></dialog>

<script>
const ESC = '\uE00C';
promise_test(async (t) => {
  const dialog = document.querySelector('dialog#test1');
  assert_true(dialog.open);
  assert_true(dialog.matches('[open]'));
  await new test_driver.send_keys(document.documentElement,ESC);
  assert_false(dialog.open);
  assert_false(dialog.matches('[open]'));
  dialog.showModal();
  assert_true(dialog.open);
  assert_true(dialog.matches('[open]'));
  await new test_driver.send_keys(document.documentElement,ESC);
  assert_false(dialog.open);
  assert_false(dialog.matches('[open]'));
}, `Dialogs that start open and have closedby should still function`);
</script>

<dl>
  <dt contenteditable></dt>
  <dialog id=test2 open></dialog>
</dl>

<script>
promise_test(async (t) => {
  // This test case is pulled from `dialog-closewatcher-crash.html`. It is
  // constructed such that this happens:
  //  1. The dialog `open` attribute is removed, which (depending on whether
  //     https://github.com/whatwg/html/pull/10124 behavior is happening) calls
  //     the close() steps.
  //  2. the last step of close() is to restore focus to the previously-focused
  //     element.
  //  3. Changing focus triggers the `focusin` event, which calls `showModal()`.
  //  4. In showModal(), the dialog is again made modal, re-constructing the
  //     close watcher and re-setting the `open` attribute.
  //  5. The call to close() ends.
  // After all of this, if things are working, the ESC key should still cause
  // the dialog to be closed.
  const dialog = document.querySelector('dialog#test2');
  const controller = new AbortController();
  document.querySelector('dl').addEventListener("focusin", () => {
    dialog.showModal();
  },{signal:controller.signal});
  // This will trigger the focus-the-previous-element behavior, which will fire
  // the `focusin` event.
  dialog.open = false;
  await new Promise(resolve => {
    document.defaultView.requestIdleCallback(() => {
      window.getSelection().addRange(document.createRange());
      dialog.close();
      resolve();
    });
  });
  assert_true(dialog.open);
  assert_true(dialog.matches('[open]'));
  // Stop re-running showModal(), so we can check that the dialog closes with ESC:
  controller.abort();
  await test_driver.send_keys(document.documentElement,ESC);
  assert_false(dialog.open,'ESC should still work');
  assert_false(dialog.matches('[open]'));
}, `Opening and closing a dialog during the dialog focus fixup should still leave closedby functional`);
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.dl.child.invalid",
      "message": "Element “dialog” not allowed as child of “dl” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 1255,
        "byte_start": 1233,
        "col": 3,
        "line": 34
      }
    },
    {
      "category": "Html",
      "code": "html.dl.missing_dd",
      "message": "Element “dl” is missing a required instance of one or more of the following child elements: “dd”.",
      "severity": "Warning",
      "span": {
        "byte_end": 1270,
        "byte_start": 1265,
        "col": 1,
        "line": 35
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
  "source_name": "html/semantics/interactive-elements/the-dialog-element/dialog-closedby-start-open.html"
}
```
