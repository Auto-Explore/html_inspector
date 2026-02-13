# html/semantics/interactive-elements/the-dialog-element/dialog-showModal-remove.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/interactive-elements/the-dialog-element/dialog-showModal-remove.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>dialog element: removing from document after showModal()</title>
<link rel=help href="https://html.spec.whatwg.org/multipage/interactive-elements.html#dom-dialog-showmodal">
<link rel=help href="https://fullscreen.spec.whatwg.org/#removing-steps">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<dialog></dialog>
<script>
async_test(t => {
  const dialog = document.querySelector('dialog')
  dialog.showModal()
  assert_true(dialog.open)
  // The dialog element is now in top layer. Removing it should synchronously
  // remove it from top layer, but should leave it in a strange limbo state.
  dialog.addEventListener('close', t.unreached_func('close event'))
  dialog.remove()
  assert_true(dialog.open)
  // if an event was queued, it would fire before this timeout
  step_timeout(t.step_func_done(() => {
    assert_true(dialog.open)
    // pass if no close event was fired
  }))
})
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
  "source_name": "html/semantics/interactive-elements/the-dialog-element/dialog-showModal-remove.html"
}
```
