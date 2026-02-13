# html/semantics/popovers/togglePopover.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/popovers/togglePopover.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel=author href="mailto:jarhar@chromium.org">
<link rel=help href="https://github.com/whatwg/html/issues/9043">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<div id=popover popover=auto>popover</div>
<div id=popover2 popover=auto>popover</div>

<script>
test(() => {
  assert_false(popover.matches(':popover-open'),
    'Popover should be closed when the test starts.');

  assert_true(popover.togglePopover(),
    'togglePopover() should return true.');
  assert_true(popover.matches(':popover-open'),
    'togglePopover() should open the popover.');

  assert_true(popover.togglePopover(/*force=*/true),
    'togglePopover(true) should return true.');
  assert_true(popover.matches(':popover-open'),
    'togglePopover(true) should open the popover.');

  assert_false(popover.togglePopover(),
    'togglePopover() should return false.');
  assert_false(popover.matches(':popover-open'),
    'togglePopover() should close the popover.');

  assert_false(popover.togglePopover(/*force=*/false),
    'togglePopover(false) should return false.');
  assert_false(popover.matches(':popover-open'),
    'togglePopover(false) should close the popover.');
}, 'togglePopover should toggle the popover and return true or false as specified.');

test(() => {
  const popover = document.getElementById('popover2');
  popover.addEventListener('beforetoggle', event => event.preventDefault(), {once: true});
  assert_false(popover.togglePopover(/*force=*/true),
    'togglePopover(true) should return false when the popover does not get opened.');
  assert_false(popover.matches(':popover-open'));

  // We could also add a test for the return value of togglePopover(false),
  // but every way to prevent that from hiding the popover also throws an
  // exception, so the return value is not testable.
}, `togglePopover's return value should reflect what the end state is, not just the force parameter.`);

test(() => {
  const popover = document.createElement('div');
  document.body.appendChild(popover);

  assert_throws_dom('NotSupportedError', () => popover.togglePopover(),
    'togglePopover() should throw an exception when the element has no popover attribute.');
  assert_throws_dom('NotSupportedError', () => popover.togglePopover(true),
    'togglePopover(true) should throw an exception when the element has no popover attribute.');
  assert_throws_dom('NotSupportedError', () => popover.togglePopover(false),
    'togglePopover(false) should throw an exception when the element has no popover attribute.');

  popover.setAttribute('popover', 'auto');
  popover.remove();

  assert_throws_dom('InvalidStateError', () => popover.togglePopover(),
    'togglePopover() should throw an exception when the element is disconnected.');
  assert_throws_dom('InvalidStateError', () => popover.togglePopover(true),
    'togglePopover(true) should throw an exception when the element is disconnected.');
  assert_throws_dom('InvalidStateError', () => popover.togglePopover(false),
    'togglePopover(false) should throw an exception when the element is disconnected.');

  document.body.appendChild(popover);
  // togglePopover(false) should not throw just because the popover is already hidden.
  popover.togglePopover(false);
  popover.showPopover();
  // togglePopover(true) should not throw just because the popover is already showing.
  popover.togglePopover(true);

  // cleanup
  popover.hidePopover();
}, 'togglePopover should throw an exception when there is no popover attribute.');
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
  "source_name": "html/semantics/popovers/togglePopover.html"
}
```
