# html/semantics/interactive-elements/the-dialog-element/dialog-close-via-attribute.tentative.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/interactive-elements/the-dialog-element/dialog-close-via-attribute.tentative.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel=author href="mailto:jarhar@chromium.org">
<link rel=help href="https://github.com/whatwg/html/issues/5802">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-vendor.js"></script>

<button>button</button>
<dialog>
  <button>button in dialog</button>
</dialog>

<script>
const dialog = document.querySelector('dialog');
const button = document.querySelector('button');
const dialogbutton = document.querySelector('dialog > button');

promise_test(async t => {
  button.focus();
  dialog.showModal();
  assert_equals(document.activeElement, dialogbutton,
    '<button> in <dialog> should be focused after opening.');

  let closeFired = false;
  let cancelFired = false;
  dialog.addEventListener('close', () => closeFired = true);
  dialog.addEventListener('cancel', () => cancelFired = true);

  dialog.removeAttribute('open');
  assert_equals(document.activeElement, dialogbutton,
    '<button> in <dialog> should still be focused immediately after removing open.');
  await new Promise(resolve => t.step_timeout(resolve, 0));
  await new Promise(requestAnimationFrame);

  assert_not_equals(document.activeElement, button,
    'Previously focused element should not be focused, even after waiting for a task.');
  assert_false(dialog.matches(':modal'),
    'The dialog should not match :modal after closing.');
  assert_false(cancelFired,
    'The cancel event should not fire when removing the open attribute.');
  assert_true(closeFired,
    'The close event should be fired when removing the open attribute.');

  let buttonFiredClick = false;
  button.addEventListener('click', () => buttonFiredClick = true);
  await test_driver.click(button);
  assert_true(buttonFiredClick,
    'The page should not be inert or blocked after removing the open attribute.');
  // Clean up
  dialog.showModal();
  dialog.close();
}, 'Removing the open attribute from an open modal dialog should run the closing algorithm.');

promise_test(async t => {
  button.focus();
  dialog.show();
  assert_equals(document.activeElement, dialogbutton,
    '<button> in <dialog> should be focused after opening.');

  let closeFired = false;
  let cancelFired = false;
  dialog.addEventListener('close', () => closeFired = true);
  dialog.addEventListener('cancel', () => cancelFired = true);

  dialog.removeAttribute('open');
  assert_equals(document.activeElement, dialogbutton,
    '<button> in <dialog> should still be focused immediately after removing open.');
  await new Promise(resolve => t.step_timeout(resolve, 0));
  await new Promise(requestAnimationFrame);

  assert_not_equals(document.activeElement, button,
    'Previously focused element should not be focused, even after waiting for a task.');
  assert_false(cancelFired,
    'The cancel event should not fire when removing the open attribute.');
  assert_true(closeFired,
    'The close event should be fired when removing the open attribute.');
  // Clean up
  dialog.show();
  dialog.close();
}, 'Removing the open attribute from an open non-modal dialog should fire a close event.');
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
  "source_name": "html/semantics/interactive-elements/the-dialog-element/dialog-close-via-attribute.tentative.html"
}
```
