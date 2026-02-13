# html/semantics/popovers/popover-focus-tabindex.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/popovers/popover-focus-tabindex.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8" />
<title>Popover focus behaviors</title>
<meta name="timeout" content="long">
<link rel="author" title="Edgar Chen" href="mailto:echen@mozilla.com">
<link rel=help href="https://html.spec.whatwg.org/#flattened-tabindex-ordered-focus-navigation-scope">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-actions.js"></script>
<script src="/resources/testdriver-vendor.js"></script>
<script src="resources/popover-utils.js"></script>

<div id=focus-tabindex>
  <span tabindex=0>First other focusable element</span>
  <button popovertarget=focus-tabindex-p tabindex="1">Toggle popover</button>
  <div popover id=focus-tabindex-p>
    Popover with <button tabindex="0">focusable element</button>
  </div>
  <span tabindex=0>Second other focusable element</span>
</div>
<script>
promise_test(async t => {
  const popover = document.querySelector('#focus-tabindex>[popover]');
  t.add_cleanup(() => {
    popover.hidePopover();
  });

  const invoker = document.querySelector('#focus-tabindex>button');
  const others = document.querySelectorAll('#focus-tabindex>span');
  invoker.focus(); // Make sure button is focused.
  assert_equals(document.activeElement, invoker);
  invoker.click(); // Activate the invoker
  assert_true(popover.matches(':popover-open'), 'Popover should be invoked by invoker');
  assert_equals(document.activeElement, invoker, 'Invoker should still be focused');
  others[1].focus();
  assert_equals(document.activeElement, others[1], "Second focused element should be focused");
  await sendShiftTab();
  assert_equals(document.activeElement, others[0], 'Popover should be skipped since its invoker has different tabindex');
}, "Cases where the popover invoker has different tabindex");
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
  "source_name": "html/semantics/popovers/popover-focus-tabindex.html"
}
```
