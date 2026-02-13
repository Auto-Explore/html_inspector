# html/semantics/popovers/popover-self-invoke.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/popovers/popover-self-invoke.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<link rel="author" href="mailto:masonf@chromium.org">
<link rel=help href="https://html.spec.whatwg.org/multipage/popover.html">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-actions.js"></script>
<script src="/resources/testdriver-vendor.js"></script>
<script src="resources/popover-utils.js"></script>

<button popover id="lock" popovertarget="lock" style="display: inline">
  Tapping this should not hang
</button>

<script>
const button = document.getElementById('lock');
function runTest(activator, description) {
  promise_test(async t => {
    assert_false(button.matches(':popover-open'), 'Button should not be open initially');
    await activator();
    assert_true(button.matches(':popover-open'), 'Button should be open after touch');
    button.hidePopover(); // Cleanup
    assert_false(button.matches(':popover-open'), 'Cleanup should close the popover');
  }, `${description} on a popover button that is its own target should open it.`);
}
// Try with both mouse and touchscreen, via test_driver.Actions()
['mouse','touch'].forEach((method) => {
  runTest(() => clickOn(button, method === 'touch'), method);
});
// Also try programmatic click.
runTest(() => button.click(), 'click');
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
  "source_name": "html/semantics/popovers/popover-self-invoke.html"
}
```
