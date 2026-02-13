# html/semantics/embedded-content/media-elements/playing-the-media-resource/fragmented-mp4-end.html

Counts:
- errors: 3
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/media-elements/playing-the-media-resource/fragmented-mp4-end.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>Fragmented MP4 Play to End</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<body>
  <video id="video"
         src="/media-source/mp4/test-v-128k-320x240-24fps-8kfr.mp4"
         type="video/mp4; codecs=avc1.64000D"
         controls>
  </video>
</body>
<script>
// Test that a fragmented mp4 can be played until the end without error.
promise_test(async t => {
  const type = video.getAttribute('type');
  const canPlay = video.canPlayType(type);
  // https://html.spec.whatwg.org/multipage/media.html#dom-navigator-canplaytype
  // must return "probably" if the user agent is confident that the type
  // represents a media resource that it can render if used in with this audio
  // or video element;
  assert_implements_optional(
    canPlay == 'probably',
    `Test needs canPlayType('${type}') == 'probably'; got '${canPlay}'.`);

  // We don't include playing here since automated test harnesses may encounter
  // underflow during testing; which can generate multiple playing events.
  video.watcher = new EventWatcher(t, video, ['error', 'seeked', 'ended']);

  // Shorten test.
  video.currentTime = 1.9;
  await video.watcher.wait_for('seeked');

  await Promise.all([
    video.play(),
    video.watcher.wait_for(['ended']),
  ]);
}, 'fragmented-mp4-end');
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.parser.stray_start_tag",
      "message": "Stray start tag “script”.",
      "severity": "Error",
      "span": {
        "byte_end": 374,
        "byte_start": 366,
        "col": 1,
        "line": 13
      }
    },
    {
      "category": "Html",
      "code": "html.parser.nonspace_after_body",
      "message": "Non-space character after body.",
      "severity": "Error",
      "span": {
        "byte_end": 1381,
        "byte_start": 374,
        "col": 9,
        "line": 13
      }
    },
    {
      "category": "Html",
      "code": "html.parser.end_tag_after_body_closed",
      "message": "Saw an end tag after “body” had been closed.",
      "severity": "Error",
      "span": {
        "byte_end": 1390,
        "byte_start": 1381,
        "col": 1,
        "line": 39
      }
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/semantics/embedded-content/media-elements/playing-the-media-resource/fragmented-mp4-end.html"
}
```
