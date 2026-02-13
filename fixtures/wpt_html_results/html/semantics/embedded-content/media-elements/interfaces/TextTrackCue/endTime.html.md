# html/semantics/embedded-content/media-elements/interfaces/TextTrackCue/endTime.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/media-elements/interfaces/TextTrackCue/endTime.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>TextTrackCue.endTime</title>
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
    var c1 = new VTTCue(-2, -1, 'text1');
    assert_equals(c1.endTime, -1);
    c1.endTime = c1.endTime;
    assert_equals(c1.endTime, -1);
    assert_throws_js(TypeError, function(){ c1.endTime = NaN; });
    c1.endTime = +Infinity;
    assert_equals(c1.endTime, +Infinity);
    assert_throws_js(TypeError, function(){ c1.endTime = -Infinity; });
}, document.title+', script-created cue');

var t_parsed = async_test(document.title+', parsed cue');
t_parsed.step(function(){
    var t = document.createElement('track');
    t.onload = this.step_func(function(){
        var c = t.track.cues;
        assert_equals(c[0].endTime, 0.001);
        assert_equals(c[1].endTime, 3600.001);
        this.done();
    });
    t.onerror = this.step_func(function() {
      assert_unreached('got error event');
    });
    t.src = 'data:text/vtt,'+encodeURIComponent('WEBVTT\n\n00:00:00.000 --> 00:00:00.001\ntest'+
                                                      '\n\nfoobar\n01:00:00.000 --> 01:00:00.001\ntest');
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
  "source_name": "html/semantics/embedded-content/media-elements/interfaces/TextTrackCue/endTime.html"
}
```
