# html/semantics/interactive-elements/the-dialog-element/dialog-closedby-show-stacked.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/interactive-elements/the-dialog-element/dialog-closedby-show-stacked.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<link rel="author" href="mailto:wpt@keithcirkel.co.uk">
<link rel=help href="https://html.spec.whatwg.org/multipage/interactive-elements.html#dialog-light-dismiss">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-actions.js"></script>
<script src="/resources/testdriver-vendor.js"></script>
<script src="../../popovers/resources/popover-utils.js"></script>

<button id="outside">Outside</button>
<dialog id=outer closedby="closerequest">
  <dialog id=inner>
  </dialog>
</dialog>

<script>
function awaitEvent(el, type, signal) {
  const {promise, resolve} = Promise.withResolvers();
  el.addEventListener(type, resolve, { once: true, signal });
  return promise
}

promise_test(async (t) => {
  inner.setAttribute('closedby', 'none');
  outer.show();
  // Need to ensure that CloseWatcher does not collapse
  // both of these shows into a single CloseWatcher group.
  await test_driver.bless();
  inner.show();
  assert_true(inner.open, "The inner dialog is open");
  assert_true(outer.open, "The outer dialog is open");

  let cancelFired = false;
  inner.addEventListener('cancel', () => cancelFired = true, t.get_signal());
  outer.addEventListener('cancel', () => cancelFired = true, t.get_signal());

  const ESC = '\uE00C';
  await test_driver.send_keys(document.documentElement,ESC);

  assert_false(cancelFired, "The cancel event was not fired");
  assert_true(inner.open, "The inner dialog is still open");
  assert_true(outer.open, "The outer dialog is still open");
},'With an inner closedby=none, the outer & inner dialogs stays open when Esc is pressed');

promise_test(async (t) => {
  inner.setAttribute('closedby', 'none');
  outer.show();
  // Need to ensure that CloseWatcher does not collapse
  // both of these shows into a single CloseWatcher group.
  await test_driver.bless();
  inner.show();
  assert_true(inner.open, "The inner dialog is open");
  assert_true(outer.open, "The outer dialog is open");

  let cancelFired = false;
  inner.addEventListener('cancel', () => cancelFired = true, t.get_signal());
  outer.addEventListener('cancel', () => cancelFired = true, t.get_signal());

  // Try clicking outside
  await clickOn(outside);

  assert_false(cancelFired, "The cancel event was not fired");
  assert_true(inner.open, "The inner dialog is open");
  assert_true(outer.open, "The outer dialog is open");
},'With an inner closedby=none, the outer & inner dialogs stays open when clicked outside');

promise_test(async (t) => {
  inner.setAttribute('closedby', 'any');
  outer.show();
  // Need to ensure that CloseWatcher does not collapse
  // both of these shows into a single CloseWatcher group.
  await test_driver.bless();
  inner.show();
  assert_true(inner.open, "The inner dialog is open");
  assert_true(outer.open, "The outer dialog is open");

  let innerCancelled = false;
  let outerCancelled = false;
  inner.addEventListener('cancel', () => innerCancelled = true, t.get_signal());
  outer.addEventListener('cancel', () => outerCancelled = true, t.get_signal());

  let innerClosed = awaitEvent(inner, 'close', t.get_signal());

  // Try clicking outside
  const ESC = '\uE00C';
  await test_driver.send_keys(document.documentElement,ESC);

  await innerClosed;

  assert_false(outerCancelled, "The outer cancel event was not fired");
  assert_true(innerCancelled, "The inner cancel event was fired");

  assert_false(inner.open, "The inner dialog is NOT open");
  assert_true(outer.open, "The outer dialog is open");
},'With an inner closedby=any, the outer dialog stays open but the inner dialogs should close, when Esc is pressed');

promise_test(async (t) => {
  inner.setAttribute('closedby', 'any');
  outer.show();
  // Need to ensure that CloseWatcher does not collapse
  // both of these shows into a single CloseWatcher group.
  await test_driver.bless();
  inner.show();
  assert_true(inner.open, "The inner dialog is open");
  assert_true(outer.open, "The outer dialog is open");

  let innerCancelled = false;
  let outerCancelled = false;
  inner.addEventListener('cancel', () => innerCancelled = true, t.get_signal());
  outer.addEventListener('cancel', () => outerCancelled = true, t.get_signal());

  let innerClosed = awaitEvent(inner, 'close', t.get_signal());

  // Try clicking outside
  await clickOn(outside);

  await innerClosed;

  assert_false(outerCancelled, "The outer cancel event was not fired");
  assert_true(innerCancelled, "The inner cancel event was fired");

  assert_false(inner.open, "The inner dialog is NOT open");
  assert_true(outer.open, "The outer dialog is open");
},'With an inner closedby=any, the outer dialog stays open but the inner dialogs should close, when clicked outside');
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
  "source_name": "html/semantics/interactive-elements/the-dialog-element/dialog-closedby-show-stacked.html"
}
```
