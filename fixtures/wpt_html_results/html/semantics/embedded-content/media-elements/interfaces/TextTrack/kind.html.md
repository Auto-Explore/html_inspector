# html/semantics/embedded-content/media-elements/interfaces/TextTrack/kind.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/media-elements/interfaces/TextTrack/kind.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>TextTrack.kind</title>
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>
<div id=log></div>
<script>
test(function(){
    var video = document.createElement('video');
    var t1 = video.addTextTrack('subtitles');
    var t2 = video.addTextTrack('captions');
    var t3 = video.addTextTrack('descriptions');
    var t4 = video.addTextTrack('chapters');
    var t5 = video.addTextTrack('metadata');
    assert_equals(t1.kind, 'subtitles');
    assert_equals(t2.kind, 'captions');
    assert_equals(t3.kind, 'descriptions');
    assert_equals(t4.kind, 'chapters');
    assert_equals(t5.kind, 'metadata');
}, document.title+', addTextTrack');
test(function(){
    var track = document.createElement('track');
    track.setAttribute('kind', 'CAPTIONS');
    var t = track.track;
    assert_equals(t.kind, 'captions');
}, document.title+', track element');
test(function(){
    var track = document.createElement('track');
    track.kind = 'captions\u0000';
    assert_equals(track.track.kind, 'metadata');
}, document.title+', \\u0000');
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
  "source_name": "html/semantics/embedded-content/media-elements/interfaces/TextTrack/kind.html"
}
```
