# html/semantics/popovers/popover-light-dismiss-touch-scroll.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/popovers/popover-light-dismiss-touch-scroll.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel=author href="mailto:jarhar@chromium.org">
<link rel=help href="https://issues.chromium.org/issues/408010435">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-actions.js"></script>
<script src="/resources/testdriver-vendor.js"></script>

<div id=popover popover=auto>popover</div>
<div style="height:99999px"></div>

<script>
promise_test(async () => {
  const popover = document.getElementById('popover');
  popover.showPopover();

  await (new test_driver.Actions()
    .addPointer('finger', 'touch')
    .pointerMove(1, 100)
    .pointerDown()
    .pointerMove(1, 1)
    .pointerUp())
    .send();

  assert_not_equals(window.scrollY, 0,
    'The page should have been scrolled by touching and dragging.');
  assert_true(popover.matches(':popover-open'),
    'The popover should still be open.');
}, 'Popovers should not be light dismissed when scrolling via touch.');
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
  "source_name": "html/semantics/popovers/popover-light-dismiss-touch-scroll.html"
}
```
