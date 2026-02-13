# html/rendering/widgets/input-date-no-resize-on-hover.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/widgets/input-date-no-resize-on-hover.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>Date input should not resize on hover when using web fonts</title>
<link rel="help" href="https://crbug.com/1167555">
<link rel="author" href="mailto:xiaochengh@chromium.org">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-actions.js"></script>
<script src="/resources/testdriver-vendor.js"></script>

<input id="target" type="date" style="font-family: custom-font">

<script>
function mouseMoveToTarget(target) {
  return new test_driver.Actions().pointerMove(0, 0, {origin: target}).send();
}

promise_test(async () => {
  // Update layout before font loads
  document.body.offsetWidth;

  const font_sheet = document.createElement('style');
  font_sheet.textContent = '@font-face { font-family: custom-font; src: url(/fonts/Revalia.woff) }';
  document.body.appendChild(font_sheet);

  await document.fonts.ready;

  const target = document.getElementById('target');
  const width_before_hover = target.offsetWidth;
  await mouseMoveToTarget(target);
  const width_after_hover = target.offsetWidth;
  assert_equals(width_before_hover, width_after_hover);
});
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
  "source_name": "html/rendering/widgets/input-date-no-resize-on-hover.html"
}
```
