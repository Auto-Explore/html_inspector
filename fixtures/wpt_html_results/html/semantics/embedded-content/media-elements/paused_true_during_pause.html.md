# html/semantics/embedded-content/media-elements/paused_true_during_pause.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/media-elements/paused_true_during_pause.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<html>
 <head>
  <title>{audio,video} events - paused property</title>
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
test(function() {
  var t = async_test("audio.paused should be true during pause event");
  var a = document.getElementById("a");
  a.addEventListener("pause", function() {
    t.step(function() {
     assert_true(a.paused);
    });

    t.done();
  }, false);
  a.src = getAudioURI("/media/sound_5") + "?" + new Date() + Math.random();
  a.play().catch(() => {});
  a.pause();
}, "audio events - paused property");

test(function() {
  var t = async_test("video.paused should be true during pause event");
  var v = document.getElementById("v");
  v.addEventListener("pause", function() {
    t.step(function() {
     assert_true(v.paused);
    });

    t.done();
  }, false);
  v.src = getVideoURI("/media/movie_5") + "?" + new Date() + Math.random();
  v.play().catch(() => {});
  v.pause();
}, "video events - paused property");
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
  "source_name": "html/semantics/embedded-content/media-elements/paused_true_during_pause.html"
}
```
