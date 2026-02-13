# html/semantics/embedded-content/media-elements/networkState_during_loadstart.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/media-elements/networkState_during_loadstart.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<html>
 <head>
  <title>{audio,video}.networkState - NETWORK_LOADING</title>
  <script src="/resources/testharness.js"></script>
  <script src="/resources/testharnessreport.js"></script>
  <script src="/common/media.js"></script>
 </head>
 <body>
  <p><a href="https://html.spec.whatwg.org/multipage/#dom-media-networkstate">spec reference</a></p>
  <audio id="a" autoplay controls>
  </audio>
  <video id="v" autoplay controls>
  </video>
  <div id="log"></div>
  <script>
var ta = async_test("audioElement.networkState should be NETWORK_LOADING during loadstart event");
var a = document.getElementById("a");
a.addEventListener("loadstart", function() {
  ta.step(function() {
   assert_equals(a.networkState,
   a.NETWORK_LOADING);
  });
  ta.done();
  a.pause();
}, false);
a.src = getAudioURI("/media/sound_5") + "?" + new Date() + Math.random();

var tv = async_test("videoElement.networkState should be NETWORK_LOADING during loadstart event");
var v = document.getElementById("v");
v.addEventListener("loadstart", function() {
  tv.step(function() {
   assert_equals(v.networkState,
   v.NETWORK_LOADING);
  });
  tv.done();
  v.pause();
}, false);
v.src = getVideoURI("/media/movie_5") + "?" + new Date() + Math.random();
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
  "source_name": "html/semantics/embedded-content/media-elements/networkState_during_loadstart.html"
}
```
