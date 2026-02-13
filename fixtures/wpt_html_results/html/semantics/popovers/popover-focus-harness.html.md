# html/semantics/popovers/popover-focus-harness.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/popovers/popover-focus-harness.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8" />
<title>Popover utils - harness test</title>
<link rel="author" href="mailto:masonf@chromium.org">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-actions.js"></script>
<script src="/resources/testdriver-vendor.js"></script>
<script src="resources/popover-utils.js"></script>

<button id=button1 tabindex="0">Button1</button>
<button id=button2 tabindex="0">Button2</button>
<button id=button3 tabindex="0">Button3</button>

<script>
promise_test(async t => {
  button1.focus();
  assert_equals(document.activeElement,button1);
  await sendTab();
  assert_equals(document.activeElement,button2,'Tab should move to button 2');
  await sendShiftTab();
  assert_equals(document.activeElement,button1,'Shift-Tab should move back to button 1');
}, "Test sendShiftTab");
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
  "source_name": "html/semantics/popovers/popover-focus-harness.html"
}
```
