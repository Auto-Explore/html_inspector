# html/webappapis/animation-frames/callback-multicalls.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/webappapis/animation-frames/callback-multicalls.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>AnimationTiming Test: multiple calls to requestAnimationFrame with the same callback</title>
<link rel="author" title="Intel" href="http://www.intel.com">
<link rel="help" href="https://html.spec.whatwg.org/multipage/#dom-window-requestanimationframe">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id="log"></div>
<script>

  async_test(function(t) {
    var counter = 0;
    window.requestAnimationFrame(callback);

    function callback() {
      ++counter;
      if (counter == 2) {
        t.done();
      } else {
        window.requestAnimationFrame(callback);
      }
    };

  }, "Check that multiple calls to requestAnimationFrame with the same callback will result in multiple entries being in the list with that same callback.");

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
  "source_name": "html/webappapis/animation-frames/callback-multicalls.html"
}
```
