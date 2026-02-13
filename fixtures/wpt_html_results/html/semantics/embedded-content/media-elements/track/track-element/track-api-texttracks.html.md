# html/semantics/embedded-content/media-elements/track/track-element/track-api-texttracks.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/media-elements/track/track-element/track-api-texttracks.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE HTML>
<html>
<head>
<title>Track element - text tracks API test</title>
<link rel="help" href="https://html.spec.whatwg.org/multipage/#text-track-api">
<link rel="author" title="Hyunjin Cho">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
</head>
<body>
<h1>Track element and API Test</h1>
<div style="display:none;">
    <video id="tracktest" src="/media/movie_300.mp4">
        <track kind="subtitles" src="resources/track.en.vtt" srclang="en" label="English">
        <track kind="captions" src="resources/track.en.vtt" srclang="en" label="English with Captions">
        <track id="french" kind="subtitles" src="resources/track.fr.vtt" srclang="fr" label="Francais">
        <track kind="subtitles" src="resources/track.de.vtt" srclang="de" label="Deutsch">
    </video>
</div>
<div id="log"></div>
<script>
test(function() {
    var t1 = document.getElementById('tracktest').textTracks;
    assert_not_equals(t1, undefined, "textTracks member should not be undefined");
}, "Check the track elements");
test(function() {
    var t2 = document.getElementById('tracktest').textTracks.getTrackById("french");
    assert_not_equals(t2, undefined, "textTracks member should not be undefined");
}, "Check getTrackById method");
test(function() {
    var t3 = document.getElementById('tracktest').textTracks.length;
    assert_equals(t3, 4, "textTracks List should be 4");
}, "Count track list");
</script>
</body>
</html>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.link.href.required",
      "message": "A “link” element must have an “href” or “imagesrcset” attribute, or both.",
      "severity": "Warning",
      "span": {
        "byte_end": 201,
        "byte_start": 162,
        "col": 1,
        "line": 6
      }
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/semantics/embedded-content/media-elements/track/track-element/track-api-texttracks.html"
}
```
