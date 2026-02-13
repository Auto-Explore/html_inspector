# html/semantics/embedded-content/media-elements/event_order_loadedmetadata_loadeddata.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/media-elements/event_order_loadedmetadata_loadeddata.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<html>
 <head>
  <title>{audio,video} events - loadedmetadata, then loadeddata</title>
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
  var t = async_test("setting src attribute on autoplay audio should trigger loadedmetadata then loadeddata event");
  var a = document.getElementById("a");
  var found_loadedmetadata = false;
  a.addEventListener("error", t.unreached_func());
  a.addEventListener("loadedmetadata", t.step_func(function() {
    found_loadedmetadata = true;
  }));
  a.addEventListener("loadeddata", t.step_func(function() {
    assert_true(found_loadedmetadata);
    t.done();
    a.pause();
  }), false);
  a.src = getAudioURI("/media/sound_5") + "?" + new Date() + Math.random();
}, "audio events - loadedmetadata, then loadeddata");

test(function() {
  var t = async_test("setting src attribute on autoplay video should trigger loadedmetadata then loadeddata event");
  var v = document.getElementById("v");
  var found_loadedmetadata = false;
  v.addEventListener("error", t.unreached_func());
  v.addEventListener("loadedmetadata", t.step_func(function() {
    found_loadedmetadata = true;
  }));
  v.addEventListener("loadeddata", t.step_func(function() {
    assert_true(found_loadedmetadata);
    t.done();
    v.pause();
  }), false);
  v.src = getVideoURI("/media/movie_5") + "?" + new Date() + Math.random();
}, "video events - loadedmetadata, then loadeddata");
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
  "source_name": "html/semantics/embedded-content/media-elements/event_order_loadedmetadata_loadeddata.html"
}
```
