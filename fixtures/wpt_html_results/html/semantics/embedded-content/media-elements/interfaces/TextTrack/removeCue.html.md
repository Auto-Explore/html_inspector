# html/semantics/embedded-content/media-elements/interfaces/TextTrack/removeCue.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/media-elements/interfaces/TextTrack/removeCue.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>TextTrack.removeCue()</title>
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
    assert_throws_dom("NOT_FOUND_ERR", function() {
        t1.removeCue(c1);
    }, 'standalone');
    t1.addCue(c1);
    assert_throws_dom("NOT_FOUND_ERR", function() {
        t2.removeCue(c1);
    }, 'listed in t1, remove from t2');
    t1.removeCue(c1);
    assert_throws_dom("NOT_FOUND_ERR", function() {
        t1.removeCue(c1);
    }, 'standalone, remove from t1');
    assert_throws_dom("NOT_FOUND_ERR", function() {
        t2.removeCue(c1);
    }, 'standalone, remove from t2');
}, document.title+', two elementless tracks');
var t = async_test(document.title+', cue from track element');
t.step(function(){
    var t1 = video.addTextTrack('subtitles');
    var track = document.createElement('track');
    track.onload = t.step_func(function(){
        var cue = track.track.cues[0];
        assert_throws_dom('NOT_FOUND_ERR', function() { t1.removeCue(cue); }, 'listed in track.track, remove from t1');
        track.track.removeCue(cue);
        assert_throws_dom('NOT_FOUND_ERR', function() { track.track.removeCue(cue); }, 'standalone, remove from track.track');
        assert_throws_dom('NOT_FOUND_ERR', function() { t1.removeCue(cue); }, 'standalone, remove from t1');
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
  "source_name": "html/semantics/embedded-content/media-elements/interfaces/TextTrack/removeCue.html"
}
```
