# html/semantics/embedded-content/media-elements/interfaces/TextTrackList/onaddtrack.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/media-elements/interfaces/TextTrackList/onaddtrack.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>TextTrackList.onaddtrack</title>
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>
<div id=log></div>
<script>
setup(function(){
    window.tracks = document.createElement('video').textTracks;
    window.ev = new Event('addtrack');
    window.ran = false;
    window.cb = function() { ran = true; };
});
test(function(){
    assert_equals(tracks.onaddtrack, null);
    tracks.onaddtrack = cb;
    assert_equals(tracks.onaddtrack, cb);
    tracks.dispatchEvent(ev);
    assert_true(ran);
    tracks.onaddtrack = null;
    ran = false;
    tracks.dispatchEvent(ev);
    assert_false(ran);
});
test(function(){
    tracks.addEventListener('addtrack', cb, false);
    tracks.dispatchEvent(ev);
    assert_true(ran);
    tracks.removeEventListener('addtrack', cb, false);
    ran = false;
    tracks.dispatchEvent(ev);
    assert_false(ran);
}, 'TextTrackList.addEventListener/removeEventListener');
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
  "source_name": "html/semantics/embedded-content/media-elements/interfaces/TextTrackList/onaddtrack.html"
}
```
