# html/semantics/embedded-content/media-elements/interfaces/TextTrackList/getter.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/media-elements/interfaces/TextTrackList/getter.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>TextTrackList getter</title>
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>
<div id=log></div>
<script>
setup(function(){
    window.video = document.createElement('video');
    video.addTextTrack('subtitles', 'b');
    window.track = document.createElement('track');
    track.label = 'a';
    video.appendChild(track);
    video.addTextTrack('subtitles', 'c');
});
test(function(){
    assert_equals(video.textTracks[0].label, 'a');
    assert_equals(video.textTracks[1].label, 'b');
    assert_equals(video.textTracks[2].label, 'c');
});
test(function(){
    var track_before = video.textTracks[0];
    video.textTracks[0] = 'foo';
    assert_equals(video.textTracks[0], track_before);
}, document.title+', no indexed set/create');
test(function(){
    'use strict';
    var track_before = video.textTracks[0];
    assert_throws_js(TypeError, function(){ video.textTracks[0] = 'foo'; });
    assert_equals(video.textTracks[0], track_before);
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
  "source_name": "html/semantics/embedded-content/media-elements/interfaces/TextTrackList/getter.html"
}
```
