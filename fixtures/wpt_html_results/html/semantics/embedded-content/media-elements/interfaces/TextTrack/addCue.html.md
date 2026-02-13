# html/semantics/embedded-content/media-elements/interfaces/TextTrack/addCue.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/media-elements/interfaces/TextTrack/addCue.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>TextTrack.addCue()</title>
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>
<div id=log></div>
<script>
setup(function(){
    window.video = document.createElement('video');
    document.body.appendChild(video);
});
test(function() {
    var t1 = video.addTextTrack('subtitles');
    var t2 = video.addTextTrack('subtitles');
    var c1 = new VTTCue(0, 1, 'text1');
    t1.addCue(c1);
    t2.addCue(c1);
    assert_equals(c1.track, t2);
}, document.title+', adding a cue to two different tracks');
test(function() {
    var t1 = video.addTextTrack('subtitles');
    var c1 = new VTTCue(0, 1, 'text1');
    t1.addCue(c1);
    assert_equals(c1.track, t1);
    t1.addCue(c1);
    assert_equals(c1.track, t1);
}, document.title+', adding a cue to a track twice');
test(function() {
    var t1 = video.addTextTrack('subtitles');
    var t2 = video.addTextTrack('subtitles');
    var c1 = new VTTCue(0, 1, 'text1');
    t1.addCue(c1);
    assert_equals(c1.track, t1);
    t1.removeCue(c1);
    assert_equals(c1.track, null);
    t2.addCue(c1);
    assert_equals(c1.track, t2);
}, document.title+', adding a removed cue to a different track');
test(function() {
    var t1 = video.addTextTrack('subtitles');
    var c1 = new VTTCue(0, 1, 'text1');
    t1.addCue(c1);
    assert_equals(t1.cues.length, 1, 't1.cues.length after first addition');
    t1.removeCue(c1);
    assert_equals(t1.cues.length, 0, 't1.cues.length after removal');
    t1.addCue(c1);
    assert_equals(t1.cues.length, 1, 't1.cues.length after second addition');
}, document.title+', adding an associated but removed cue to the same track');

var t = async_test(document.title+', adding a cue associated with a track element to other track');
t.step(function(){
    var t1 = video.addTextTrack('subtitles');
    var track = document.createElement('track');
    track.onload = t.step_func(function(){
        var cue = track.track.cues[0];
        track.track.removeCue(cue);
        t1.addCue(cue);
        assert_equals(cue.track, t1);
        t.done();
    });
    track.onerror = t.step_func(function() {
      assert_unreached('got error event');
    });
    track.src= 'data:text/vtt,'+encodeURIComponent('WEBVTT\n\n00:00:00.000 --> 00:00:01.000\ntest\n');
    track.kind = 'subtitles';
    track.track.mode = 'hidden';
    video.appendChild(track);
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
  "source_name": "html/semantics/embedded-content/media-elements/interfaces/TextTrack/addCue.html"
}
```
