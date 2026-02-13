# html/webappapis/animation-frames/cancel-pending.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/webappapis/animation-frames/cancel-pending.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>cancelAnimationFrame cancels a pending animation frame callback</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<link rel="help" href="https://html.spec.whatwg.org/multipage/imagebitmap-and-animations.html#run-the-animation-frame-callbacks">
<div id="log"></div>
<script>
async_test(t => {
  let didCall = false;

  function callbackOne() {
    cancelAnimationFrame(twoHandle);
    requestAnimationFrame(t.step_func(() => {
      assert_false(didCall, 'Should NOT have called the second callback');
      t.done();
    }));
  }

  function callbackTwo() {
    didCall = true;
  }

  requestAnimationFrame(callbackOne);
  const twoHandle = requestAnimationFrame(callbackTwo);
}, 'cancelAnimationFrame cancels a pending animation frame callback');
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
  "source_name": "html/webappapis/animation-frames/cancel-pending.html"
}
```
