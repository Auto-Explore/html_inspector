# html/rendering/replaced-elements/embedded-content/audio-controls-intrinsic-size.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/replaced-elements/embedded-content/audio-controls-intrinsic-size.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>Audio intrinsic size doesn't depend on its max size</title>
<link rel="help" href="https://bugzilla.mozilla.org/show_bug.cgi?id=1683979">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div style="display: inline-block">
  <audio controls style="max-width: 99%" id="test"></audio>
</div>
<script>
let audio = document.getElementById("test");

function computeSize() {
  return audio.getBoundingClientRect().width;
}

let size = computeSize();
async_test(function(t) {
  requestAnimationFrame(t.step_func(function() {
    assert_equals(computeSize(), size, "Shouldn't have changed size");
    requestAnimationFrame(t.step_func_done(function() {
      assert_equals(computeSize(), size, "Shouldn't have changed size");
    }));
  }));
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
  "source_name": "html/rendering/replaced-elements/embedded-content/audio-controls-intrinsic-size.html"
}
```
