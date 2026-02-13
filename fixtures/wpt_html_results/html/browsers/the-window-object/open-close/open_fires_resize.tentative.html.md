# html/browsers/the-window-object/open-close/open_fires_resize.tentative.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/the-window-object/open-close/open_fires_resize.tentative.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>Test that a resize event is fired on newly opened windows</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script>

promise_test(async t => {
  popup = window.open("about:blank", "_blank", "width=500,height=500");
  await new Promise(resolve => popup.onresize = resolve);
  assert_true(true, "Got resize event");
}, "Opening a popup with specified size fires a resize event");

promise_test(async t => {
  popup = window.open("about:blank", "_blank", "popup");
  await new Promise(resolve => popup.onresize = resolve);
  assert_true(true, "Got resize event");
}, "Opening a popup fires a resize event");

promise_test(async t => {
  popup = window.open("about:blank");
  await new Promise(resolve => popup.onresize = resolve);
  assert_true(true, "Got resize event");
}, "Opening a window fires a resize event");

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
  "source_name": "html/browsers/the-window-object/open-close/open_fires_resize.tentative.html"
}
```
