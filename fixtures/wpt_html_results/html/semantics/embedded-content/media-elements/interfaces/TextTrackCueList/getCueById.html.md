# html/semantics/embedded-content/media-elements/interfaces/TextTrackCueList/getCueById.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/media-elements/interfaces/TextTrackCueList/getCueById.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>TextTrackCueList.getCueById</title>
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>
<div id=log></div>
<script>
test(function(){
    var video = document.createElement('video');
    var t = video.addTextTrack('subtitles');
    document.body.appendChild(video);
    var cues = t.cues;
    var c = new VTTCue(0, 1, 'text1');
    t.addCue(c);
    assert_equals(cues.getCueById(""), null, '""');
    assert_equals(cues.getCueById(null), null, 'null');
    assert_equals(cues.getCueById(undefined), null, 'undefined');
}, document.title+ ', no id');
test(function(){
    var video = document.createElement('video');
    var t = video.addTextTrack('subtitles');
    document.body.appendChild(video);
    var cues = t.cues;
    var c = new VTTCue(0, 1, 'text1');
    c.id = 'foo';
    t.addCue(c);
    assert_equals(cues.getCueById(""), null, '""');
    assert_equals(cues.getCueById("foo"), c, '"foo"');
    assert_equals(cues.getCueById({toString:function(){return "foo"}}), c, 'object');
}, document.title+ ', id foo');
test(function(){
    var video = document.createElement('video');
    var t = video.addTextTrack('subtitles');
    document.body.appendChild(video);
    var cues = t.cues;
    var c = new VTTCue(0, 1, 'text1');
    c.id = '1';
    t.addCue(c);
    assert_equals(cues.getCueById(""), null, '""');
    assert_equals(cues.getCueById("1"), c, '"1"');
    assert_equals(cues.getCueById(1), c, '1');
}, document.title+ ', no 1');
test(function(){
    var video = document.createElement('video');
    var t = video.addTextTrack('subtitles');
    document.body.appendChild(video);
    var cues = t.cues;
    var c = new VTTCue(0, 1, 'text1');
    c.id = 'a\u0000b';
    t.addCue(c);
    assert_equals(cues.getCueById("a\u0000b"), c, '"a\\u0000b"');
    assert_equals(cues.getCueById("a"), null, '"a"');
}, document.title+ ', id a\\u0000b');
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
  "source_name": "html/semantics/embedded-content/media-elements/interfaces/TextTrackCueList/getCueById.html"
}
```
