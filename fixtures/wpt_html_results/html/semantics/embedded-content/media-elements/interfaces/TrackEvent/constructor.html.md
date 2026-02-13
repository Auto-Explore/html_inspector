# html/semantics/embedded-content/media-elements/interfaces/TrackEvent/constructor.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/media-elements/interfaces/TrackEvent/constructor.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>TrackEvent constructor</title>
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>
<div id=log></div>
<script>
test(function(){
    var ev = new TrackEvent('foo');
    assert_true(ev instanceof TrackEvent, 'ev instanceof TrackEvent');
    assert_true(ev instanceof Event, 'ev instanceof Event');
    assert_equals(ev.track, null, 'ev.track');
    ev.track = {};
    assert_equals(ev.track, null, 'ev.track after assignment');
}, document.title+', one arg');
test(function(){
    var video = document.createElement('video');
    var testTrack = video.addTextTrack('subtitles', 'foo', 'foo');
    var ev = new TrackEvent('foo', {track: testTrack});
    assert_true(ev instanceof TrackEvent, 'ev instanceof TrackEvent');
    assert_true(ev instanceof Event, 'ev instanceof Event');
    assert_equals(ev.track, testTrack, 'ev.track');
    ev.track = {};
    assert_equals(ev.track, testTrack, 'ev.track after assignment');
}, document.title+', two args');
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
  "source_name": "html/semantics/embedded-content/media-elements/interfaces/TrackEvent/constructor.html"
}
```
