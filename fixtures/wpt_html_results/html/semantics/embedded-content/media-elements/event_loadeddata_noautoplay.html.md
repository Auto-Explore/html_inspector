# html/semantics/embedded-content/media-elements/event_loadeddata_noautoplay.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/media-elements/event_loadeddata_noautoplay.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<html>
 <head>
  <title>{audio,video} events - loadeddata</title>
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
  var t = async_test("setting src attribute on non-autoplay audio should trigger loadeddata event");
  var a = document.getElementById("a");
  a.addEventListener("error", t.unreached_func());
  a.addEventListener("loadeddata", t.step_func_done(), false);
  a.src = getAudioURI("/media/sound_5") + "?" + new Date() + Math.random();
}, "audio events - loadeddata");

test(function() {
  var t = async_test("setting src attribute on non-autoplay video should trigger loadeddata event");
  var v = document.getElementById("v");
  v.addEventListener("error", t.unreached_func());
  v.addEventListener("loadeddata", t.step_func_done(), false);
  v.src = getVideoURI("/media/movie_5") + "?" + new Date() + Math.random();
}, "video events - loadeddata");
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
  "source_name": "html/semantics/embedded-content/media-elements/event_loadeddata_noautoplay.html"
}
```
