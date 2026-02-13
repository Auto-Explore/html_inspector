# html/semantics/popovers/light-dismiss-remove-target.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/popovers/light-dismiss-remove-target.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel=author href="mailto:jarhar@chromium.org">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-vendor.js"></script>
<script src="/resources/testdriver-actions.js"></script>

<div id=target>light dismiss target</div>
<div id=popover popover>popover</div>

<script>
const target = document.getElementById('target');
const popover = document.getElementById('popover');

promise_test(async () => {
  popover.showPopover();
  assert_true(popover.matches(':popover-open'),
    'popover should be open at the start of the test.');

  target.addEventListener('pointerdown', () => {
    target.remove();
  });

  await (new test_driver.Actions()
    .pointerMove(0, 0, {origin: target})
    .pointerDown()
    .pointerUp())
    .send();

  assert_true(popover.matches(':popover-open'),
    'popover should not be light dismissed after removing the target during pointerdown.');
}, 'Light dismiss should still happen if the clicked element is removed during pointerdown.');
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
  "source_name": "html/semantics/popovers/light-dismiss-remove-target.html"
}
```
