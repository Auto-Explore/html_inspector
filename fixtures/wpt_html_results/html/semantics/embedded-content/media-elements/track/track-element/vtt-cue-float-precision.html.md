# html/semantics/embedded-content/media-elements/track/track-element/vtt-cue-float-precision.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/media-elements/track/track-element/vtt-cue-float-precision.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>Float precision of VTTCue attributes line, position and size</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id=log></div>
<script>
test(function() {
    var cue = new VTTCue(0, 1, 'text');

    // Assign a value which is exactly representable as double but not float.
    var doubleValue = 1.000000000000004;
    cue.line = doubleValue;
    assert_equals(cue.line, doubleValue);
    cue.position = doubleValue;
    assert_equals(cue.position, doubleValue);
    cue.size = doubleValue;
    assert_equals(cue.size, doubleValue);

    // Assign a value which is exactly representable as float but is non-integral.
    var floatValue = 1.5;
    cue.line = floatValue;
    assert_equals(cue.line, floatValue);
    cue.position = floatValue;
    assert_equals(cue.position, floatValue);
    cue.size = floatValue;
    assert_equals(cue.size, floatValue);
}, document.title+', stored as floats');
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
  "source_name": "html/semantics/embedded-content/media-elements/track/track-element/vtt-cue-float-precision.html"
}
```
