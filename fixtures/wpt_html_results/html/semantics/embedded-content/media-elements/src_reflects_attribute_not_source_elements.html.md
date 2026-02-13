# html/semantics/embedded-content/media-elements/src_reflects_attribute_not_source_elements.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/media-elements/src_reflects_attribute_not_source_elements.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<html>
 <head>
  <title>{audio,video}.src - reflection test</title>
  <script src="/resources/testharness.js"></script>
  <script src="/resources/testharnessreport.js"></script>
 </head>
 <body>
  <p><a href="https://html.spec.whatwg.org/multipage/#dom-media-src">spec reference</a></p>
  <audio id="audio" src="foo">
   <source src="barbaz" />
  </audio>
  <video id="video" src="foo">
   <source src="barbaz" />
  </video>
  <div id="log"></div>
  <script>
test(function() {
 assert_equals(
   document.getElementById("audio").src.indexOf("barbaz"),
   -1,
   "audioElement.src should reflect src attribute, not source child elements");
}, "audio.src - reflection test");

test(function() {
 assert_equals(
   document.getElementById("video").src.indexOf("barbaz"),
   -1,
   "videoElement.src should reflect src attribute, not source child elements");
}, "video.src - reflection test");
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
  "source_name": "html/semantics/embedded-content/media-elements/src_reflects_attribute_not_source_elements.html"
}
```
