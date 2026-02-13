# html/semantics/embedded-content/media-elements/readyState_initial.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/media-elements/readyState_initial.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<html>
 <head>
  <title>{audio,video}.readyState - default state</title>
  <script src="/resources/testharness.js"></script>
  <script src="/resources/testharnessreport.js"></script>
 </head>
 <body>
  <p><a href="https://html.spec.whatwg.org/multipage/#dom-media-networkstate">spec reference</a></p>
  <audio id="a">
  </audio>
  <video id="v">
  </video>
  <div id="log"></div>
  <script>
test(function() {
 var a = document.getElementById("a");
 assert_equals(
  a.readyState,
  a.HAVE_NOTHING,
  "audioElement.readyState should be HAVE_NOTHING to begin with");
}, "audio.readyState - default state");

test(function() {
 var v = document.getElementById("v");
 assert_equals(
  v.readyState,
  v.HAVE_NOTHING,
  "videoElement.readyState should be HAVE_NOTHING to begin with");
}, "video.readyState - default state");
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
  "source_name": "html/semantics/embedded-content/media-elements/readyState_initial.html"
}
```
