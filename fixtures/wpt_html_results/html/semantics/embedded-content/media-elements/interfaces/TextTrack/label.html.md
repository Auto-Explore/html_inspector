# html/semantics/embedded-content/media-elements/interfaces/TextTrack/label.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/media-elements/interfaces/TextTrack/label.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>TextTrack.label</title>
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>
<div id=log></div>
<script>
setup(function(){
    window.video = document.createElement('video');
    window.t1 = video.addTextTrack('subtitles', 'foo');
    window.track = document.createElement('track');
    track.setAttribute('label', 'bar');
    video.appendChild(track);
    window.t2 = track.track;
});
test(function(){
    assert_equals(t1.label, 'foo');
    assert_equals(t2.label, 'bar');
    track.label = 'baz';
    assert_equals(t2.label, 'baz');
    track.removeAttribute('label');
    assert_equals(t2.label, '');
});
test(function(){
    track.label = '\u0000a';
    assert_equals(t2.label, '\u0000a');
    track.setAttribute('label', '\u0000b', 'IDL attribute');
    assert_equals(t2.label, '\u0000b', 'content attribute');
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
  "source_name": "html/semantics/embedded-content/media-elements/interfaces/TextTrack/label.html"
}
```
