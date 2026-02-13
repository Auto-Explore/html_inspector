# html/semantics/popovers/popover-focus-slotted.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/popovers/popover-focus-slotted.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8" />
<title>Popover focus with slotted popover and invoker</title>
<link rel="author" href="mailto:masonf@chromium.org">
<link rel=help href="https://crbug.com/447888734">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-actions.js"></script>
<script src="/resources/testdriver-vendor.js"></script>
<script src="/html/semantics/popovers/resources/popover-utils.js"></script>

<div id="host">
  <template shadowrootmode="open">
    <div>
      <slot id="invoker" name="invoker"></slot>
      <slot id="popover" popover="manual"></slot>
    </div>
  </template>
  <button slot="invoker">Click me</button>
  <button id="inner">Click me next</button>
</div>

<script>
  const div = document.getElementById('host');
  const invoker = div.shadowRoot.querySelector("#invoker");
  const popover = div.shadowRoot.querySelector("#popover");
  const inner = document.getElementById('inner');
  popover.togglePopover({source: invoker});

  promise_test(async () => {
    assert_true(popover.matches(':popover-open'), 'Popover should be open');
    inner.focus();
    assert_equals(document.activeElement, inner, 'Start with inner focused');

    // Tab forward
    await sendTab();
    assert_not_equals(document.activeElement, inner, 'Focus should move');
  }, 'Tabbing forward out of a <slot popover> should not hang.');

  promise_test(async () => {
    assert_true(popover.matches(':popover-open'), 'Popover should be open');
    inner.focus();
    assert_equals(document.activeElement, inner, 'Start with inner focused');

    // Tab backwards
    await sendShiftTab();
    assert_not_equals(document.activeElement, inner, 'Focus should move');
  }, 'Tabbing backwards out of a <slot popover> should not hang.');
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
  "source_name": "html/semantics/popovers/popover-focus-slotted.html"
}
```
