# html/semantics/embedded-content/media-elements/interfaces/TextTrackList/getTrackById.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/media-elements/interfaces/TextTrackList/getTrackById.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>TextTrackList.getTrackById</title>
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>
<div id=log></div>
<script>
test(function(){
    var video = document.createElement('video');
    var track1 = video.addTextTrack('subtitles');
    var track2 = video.addTextTrack('subtitles');
    assert_equals(track1.id, '');
    assert_equals(track2.id, '');
    assert_equals(video.textTracks.getTrackById(''), track1);
    assert_equals(video.textTracks.getTrackById('fake-id'), null);
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
  "source_name": "html/semantics/embedded-content/media-elements/interfaces/TextTrackList/getTrackById.html"
}
```
