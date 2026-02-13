# html/semantics/embedded-content/media-elements/readyState_during_canplaythrough.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/media-elements/readyState_during_canplaythrough.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<html>
 <head>
  <title>{audio,video} events - readyState property during canplaythrough</title>
  <script src="/resources/testharness.js"></script>
  <script src="/resources/testharnessreport.js"></script>
  <script src="/common/media.js"></script>
 </head>
 <body>
  <p><a href="https://html.spec.whatwg.org/multipage/#mediaevents">spec reference</a></p>
  <audio id="a" autoplay controls>
  </audio>
  <video id="v" autoplay controls>
  </video>
  <div id="log"></div>
  <script>
test(function() {
  var t = async_test("audio.readyState should be HAVE_ENOUGH_DATA during canplaythrough event");
  var a = document.getElementById("a");
  a.addEventListener("error", t.unreached_func());
  a.addEventListener("canplaythrough", t.step_func(function() {
    assert_equals(a.readyState, a.HAVE_ENOUGH_DATA);
    t.done();
    a.pause();
  }), false);
  a.src = getAudioURI("/media/sound_5") + "?" + new Date() + Math.random();
}, "audio events - readyState property during canplaythrough");

test(function() {
  var t = async_test("video.readyState should be HAVE_ENOUGH_DATA during canplaythrough event");
  var v = document.getElementById("v");
  v.addEventListener("error", t.unreached_func());
  v.addEventListener("canplaythrough", t.step_func(function() {
    assert_equals(v.readyState, v.HAVE_ENOUGH_DATA);
    t.done();
    v.pause();
  }), false);
  v.src = getVideoURI("/media/movie_5") + "?" + new Date() + Math.random();
}, "video events - readyState property during canplaythrough");
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
  "source_name": "html/semantics/embedded-content/media-elements/readyState_during_canplaythrough.html"
}
```
