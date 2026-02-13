# html/semantics/embedded-content/media-elements/event_order_durationchange_resize_loadedmetadata.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/media-elements/event_order_durationchange_resize_loadedmetadata.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<html>
 <head>
  <title>{audio,video} events - durationchange, resize (only video), then loadedmetadata</title>
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
  var t = async_test("setting src attribute on autoplay audio should trigger durationchange then loadedmetadata event");
  var a = document.getElementById("a");
  var found_durationchange = false;
  a.addEventListener("error", t.unreached_func());
  a.addEventListener("durationchange", t.step_func(function() {
    found_durationchange = true;
  }));
  a.addEventListener("loadedmetadata", t.step_func(function() {
    assert_true(found_durationchange);
    t.done();
    a.pause();
  }), false);
  a.src = getAudioURI("/media/sound_5") + "?" + new Date() + Math.random();
}, "audio events - durationchange, then loadedmetadata");

test(function() {
  var t = async_test("setting src attribute on autoplay video should trigger durationchange, resize then loadedmetadata event");
  var v = document.getElementById("v");
  var found_durationchange = false;
  var found_resize = false;
  v.addEventListener("error", t.unreached_func());
  v.addEventListener("durationchange", t.step_func(function() {
    found_durationchange = true;
  }));
  v.addEventListener("resize", t.step_func(function() {
    found_resize = true;
  }));
  v.addEventListener("loadedmetadata", t.step_func(function() {
    assert_true(found_durationchange);
    assert_true(found_resize);
    t.done();
    v.pause();
  }), false);
  v.src = getVideoURI("/media/movie_5") + "?" + new Date() + Math.random();
}, "video events - durationchange, resize then loadeddata");
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
  "source_name": "html/semantics/embedded-content/media-elements/event_order_durationchange_resize_loadedmetadata.html"
}
```
