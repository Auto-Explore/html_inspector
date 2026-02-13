# html/semantics/embedded-content/media-elements/track/track-element/track-data-url.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/media-elements/track/track-element/track-data-url.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>track element data: URL</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id=log></div>
<script>
[null, "anonymous", "use-credentials"].forEach(function(crossOriginValue) {
  async_test(function() {
    var video = document.createElement('video');
    if (crossOriginValue !== null) {
      video.setAttribute('crossorigin', crossOriginValue);
    }
    document.body.appendChild(video);
    var t = document.createElement('track');
    t.onload = this.step_func_done(function() {
      assert_equals(t.track.cues.length, 1);
      assert_equals(t.track.cues[0].startTime, 1);
      assert_equals(t.track.cues[0].endTime, 2);
      assert_equals(t.track.cues[0].id, 'x');
      assert_equals(t.track.cues[0].text, 'test');
    });
    t.onerror = this.step_func(function() {
      assert_unreached('got error event');
    });
    t.src = 'data:text/vtt,'+encodeURIComponent('WEBVTT\n\nx\n00:00:01.000 --> 00:00:02.000\ntest\n\n');
    t.track.mode = 'showing';
    video.appendChild(t);
  }, document.title + ' ' + (crossOriginValue ? crossOriginValue : 'No CORS'));
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
  "source_name": "html/semantics/embedded-content/media-elements/track/track-element/track-data-url.html"
}
```
