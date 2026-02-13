# html/semantics/embedded-content/media-elements/track/track-element/src-clear-cues.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/media-elements/track/track-element/src-clear-cues.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>track element changing "track URL" and clearing cues</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id=log></div>
<script>
(async_test(document.title+', set mode, add cue, set src')).step(function(){
    var track = document.createElement('track');
    var c = new VTTCue(0, 1, 'foo');
    c.id = 'id';
    track.track.addCue(c);
    assert_equals(track.track.cues, null, 'cues before setting src or mode');
    track.track.mode = 'showing';
    assert_equals(track.track.cues.length, 1, 'cues after setting mode');
    var cues = track.track.cues;
    track.src = 'data:,a';
    assert_equals(track.track.cues.length, 0, 'cues.length after setting src');
    assert_equals(track.track.cues, cues, 'track.track.cues sameness after setting src');
    assert_equals(c.id, 'id', 'liveness of removed cue');
    this.done();
});

(async_test(document.title+', set mode, set src, add cue, change src')).step(function(){
    var track = document.createElement('track');
    track.track.mode = 'showing';
    track.src = 'data:,a';
    var c = new VTTCue(0, 1, 'foo');
    c.id = 'id';
    track.track.addCue(c);
    assert_equals(track.track.cues.length, 1, 'cues.length before changing src');
    var cues = track.track.cues;
    track.src = 'data:,b';
    assert_equals(track.track.cues.length, 0, 'cues.length after changing src');
    assert_equals(track.track.cues, cues, 'track.track.cues sameness after changing src');
    assert_equals(c.id, 'id', 'liveness of removed cue');
    this.done();
});

(async_test(document.title+', set mode, add cue, change mode to disabled, set src')).step(function(){
    var track = document.createElement('track');
    track.track.mode = 'showing';
    var c = new VTTCue(0, 1, 'foo');
    c.id = 'id';
    track.track.addCue(c);
    var cues = track.track.cues;
    track.track.mode = 'disabled';
    track.src = 'data:,a';
    assert_equals(cues.length, 0, 'cues.length after changing src');
    assert_equals(c.id, 'id', 'liveness of removed cue');
    this.done();
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
  "source_name": "html/semantics/embedded-content/media-elements/track/track-element/src-clear-cues.html"
}
```
