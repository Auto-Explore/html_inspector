# html/semantics/embedded-content/media-elements/preload_reflects_none_autoplay.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/media-elements/preload_reflects_none_autoplay.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<html>
 <head>
  <title>{audio,video}.preload - reflection test</title>
  <script src="/resources/testharness.js"></script>
  <script src="/resources/testharnessreport.js"></script>
 </head>
 <body>
  <p><a href="https://html.spec.whatwg.org/multipage/#dom-media-preload">spec reference</a></p>
  <audio id="audio" autoplay preload="none">
  </audio>
  <video id="video" autoplay preload="none">
  </video>
  <div id="log"></div>
  <script>
test(function() {
 assert_equals(
  document.getElementById("audio").preload,
  "none",
  "audioElement.preload reflects 'none' value even if autoplay attribute is present");
}, "audio.preload - reflection test");

test(function() {
 assert_equals(
  document.getElementById("video").preload,
  "none",
  "videoElement.preload reflects 'none' value even if autoplay attribute is present");
}, "video.preload - reflection test");
  </script>
 </body>
</html>
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
  "source_name": "html/semantics/embedded-content/media-elements/preload_reflects_none_autoplay.html"
}
```
