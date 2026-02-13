# html/semantics/embedded-content/media-elements/interfaces/TextTrack/language.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/media-elements/interfaces/TextTrack/language.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>TextTrack.language</title>
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>
<div id=log></div>
<script>
setup(function(){
    window.video = document.createElement('video');
    window.t1 = video.addTextTrack('subtitles', 'foo', 'foo');
    window.track = document.createElement('track');
    track.setAttribute('srclang', 'bar');
    video.appendChild(track);
    window.t2 = track.track;
});
test(function(){
    assert_equals(t1.language, 'foo');
    assert_equals(t2.language, 'bar');
    track.srclang = 'baz';
    assert_equals(t2.language, 'baz');
    track.removeAttribute('srclang');
    assert_equals(t2.language, '');
});
test(function(){
    track.srclang = '\u0000a';
    assert_equals(t2.language, '\u0000a', 'IDL attribute');
    track.setAttribute('srclang', '\u0000b');
    assert_equals(t2.language, '\u0000b', 'content attribute');
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
  "source_name": "html/semantics/embedded-content/media-elements/interfaces/TextTrack/language.html"
}
```
