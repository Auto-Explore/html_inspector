# html/semantics/embedded-content/media-elements/interfaces/TextTrackCue/pauseOnExit.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/media-elements/interfaces/TextTrackCue/pauseOnExit.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>TextTrackCue.pauseOnExit</title>
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>
<div id=log></div>
<script>
setup(function(){
    window.video = document.createElement('video');
    window.t1 = video.addTextTrack('subtitles');
    document.body.appendChild(video);
});
test(function(){
    var c1 = new VTTCue(0, 1, 'text1');
    assert_equals(c1.pauseOnExit, false);
    c1.pauseOnExit = null;
    assert_equals(c1.pauseOnExit, false);
    c1.pauseOnExit = 'foo';
    assert_equals(c1.pauseOnExit, true);
}, document.title+', script-created cue');

var t_parsed = async_test(document.title+', parsed cue');
t_parsed.step(function(){
    var t = document.createElement('track');
    t.onload = this.step_func(function(){
        var c1 = t.track.cues[0];
        assert_equals(c1.pauseOnExit, false);
        c1.pauseOnExit = null;
        assert_equals(c1.pauseOnExit, false);
        c1.pauseOnExit = 'foo';
        assert_equals(c1.pauseOnExit, true);
        this.done();
    });
    t.onerror = this.step_func(function() {
      assert_unreached('got error event');
    });
    t.src = 'data:text/vtt,'+encodeURIComponent('WEBVTT\n\n00:00:00.000 --> 00:00:00.001\ntest');
    t.track.mode = 'showing';
    video.appendChild(t);
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
  "source_name": "html/semantics/embedded-content/media-elements/interfaces/TextTrackCue/pauseOnExit.html"
}
```
