# html/interaction/focus/focus-input-type-switch.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/interaction/focus/focus-input-type-switch.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>Inputs remain focusable upon changing type</title>
<link rel="help" href="https://wicg.github.io/auxclick">
<link rel="help" href="https://bugzilla.mozilla.org/show_bug.cgi?id=981248">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-actions.js"></script>
<script src="/resources/testdriver-vendor.js"></script>
<h1>Can still focus on inputs that change types</h1>
<input type="text" value="123" onfocus="javascript:event.target.type='number'"
                               onblur="javascript:event.target.type='text'">
<script>
promise_test(() => {
  // Click the input to attempt to focus on it
  const target = document.querySelector("input");
  const actions = new test_driver.Actions();
  return actions.pointerMove(0, 0, {origin: target})
         .pointerDown({button: actions.ButtonType.LEFT})
         .pointerUp({button: actions.ButtonType.LEFT})
         .send()
         .then(() => assert_equals(document.activeElement, target,
                                   "The element was correctly focused"));
}, "Can change an input's type during focus handler without breaking focus");
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
  "source_name": "html/interaction/focus/focus-input-type-switch.html"
}
```
