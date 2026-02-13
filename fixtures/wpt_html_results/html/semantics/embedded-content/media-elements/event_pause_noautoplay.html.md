# html/semantics/embedded-content/media-elements/event_pause_noautoplay.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/media-elements/event_pause_noautoplay.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<html>
 <head>
  <title>{audio,video} events - pause</title>
  <script src="/resources/testharness.js"></script>
  <script src="/resources/testharnessreport.js"></script>
  <script src="/common/media.js"></script>
 </head>
 <body>
  <p><a href="https://html.spec.whatwg.org/multipage/#mediaevents">spec reference</a></p>
  <audio id="a" controls>
  </audio>
  <video id="v" controls>
  </video>
  <div id="log"></div>
  <script>
promise_test(function(t) {
  var async_t = async_test("calling play() then pause() on non-autoplay audio should trigger pause event");
  var a = document.getElementById("a");
  a.addEventListener("pause", function() {
    async_t.done();
  }, false);
  a.src = getAudioURI("/media/sound_5") + "?" + new Date() + Math.random();
  var play_promise = a.play();
  a.pause();
  return promise_rejects_dom(t, "AbortError", play_promise, "pause() should reject all pending play Promises");
}, "audio events - pause");

promise_test(function(t) {
  var async_t = async_test("calling play() then pause() on non-autoplay video should trigger pause event");
  var v = document.getElementById("v");
  v.addEventListener("pause", function() {
    async_t.done();
  }, false);
  v.src = getVideoURI("/media/movie_5") + "?" + new Date() + Math.random();
  var play_promise = v.play()
  v.pause();
  return promise_rejects_dom(t, "AbortError", play_promise, "pause() should reject all pending play Promises");
}, "video events - pause");
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
  "source_name": "html/semantics/embedded-content/media-elements/event_pause_noautoplay.html"
}
```
