# html/semantics/embedded-content/media-elements/interfaces/TextTrackCueList/getter.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/media-elements/interfaces/TextTrackCueList/getter.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>TextTrackCueList getter</title>
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
    var cues = t1.cues;
    assert_equals(cues[0], undefined, 'cues[0] before');
    var c1 = new VTTCue(0, 1, 'text1');
    t1.addCue(c1);
    assert_equals(cues[0], c1, 'cues[0]');
    assert_equals(cues[1], undefined, 'cues[1]');
    assert_equals(cues[-1], undefined, 'cues[-1]');
    t1.removeCue(c1);
    assert_equals(cues[0], undefined, 'cues[0] after');
});
test(function(){
    var cues = t1.cues;
    assert_equals(cues[0], undefined);
    cues[0] = 'foo';
    assert_equals(cues[0], undefined);
    var c1 = new VTTCue(0, 1, 'text1');
    t1.addCue(c1);
    assert_equals(cues[0], c1);
    cues[0] = 'foo';
    assert_equals(cues[0], c1);
    t1.removeCue(c1);
}, document.title+', no indexed set/create');
test(function(){
    'use strict';
    var cues = t1.cues;
    assert_equals(cues[0], undefined);
    assert_throws_js(TypeError, function() { cues[0] = 'foo'; });
    assert_equals(cues[0], undefined);
    var c1 = new VTTCue(0, 1, 'text1');
    t1.addCue(c1);
    assert_equals(cues[0], c1);
    assert_throws_js(TypeError, function() { cues[0] = 'foo'; });
    assert_equals(cues[0], c1);
    t1.removeCue(c1);
}, document.title+', no indexed set/create (strict)');

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
  "source_name": "html/semantics/embedded-content/media-elements/interfaces/TextTrackCueList/getter.html"
}
```
