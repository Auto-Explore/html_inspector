# html/webappapis/animation-frames/callback-timestamp.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/webappapis/animation-frames/callback-timestamp.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>AnimationTiming Test: FrameRequestCallback - timestamp argument</title>
<link rel="author" title="Intel" href="http://www.intel.com">
<link rel="help" href="https://html.spec.whatwg.org/multipage/#animation-frames">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id="log"></div>
<script>

  async_test(t => {
    requestAnimationFrame(t.step_func_done(time => {
      assert_equals(typeof time, "number", "callback contains a number argument");
    }))
  }, "Check FrameRequestCallback has a DOMHighResTimeStamp argument");

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
  "source_name": "html/webappapis/animation-frames/callback-timestamp.html"
}
```
