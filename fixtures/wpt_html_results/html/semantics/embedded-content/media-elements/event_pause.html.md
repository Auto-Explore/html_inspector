# html/semantics/embedded-content/media-elements/event_pause.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/media-elements/event_pause.html",
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
  <audio id="a" autoplay controls>
  </audio>
  <video id="v" autoplay controls>
  </video>
  <div id="log"></div>
  <script>
test(function() {
  var t = async_test("calling pause() on autoplay audio should trigger pause event");
  var a = document.getElementById("a");
  a.addEventListener("error", t.unreached_func());
  a.addEventListener("pause", t.step_func_done(), false);
  a.addEventListener("play", t.step_func(function() {
    a.pause(); // pause right after play
  }));
  a.src = getAudioURI("/media/sound_5") + "?" + new Date() + Math.random();
}, "audio events - pause");

test(function() {
  var t = async_test("calling pause() on autoplay video should trigger pause event");
  var v = document.getElementById("v");
  v.addEventListener("error", t.unreached_func());
  v.addEventListener("pause", t.step_func_done(), false);
  v.addEventListener("play", t.step_func(function() {
    v.pause(); // pause right after play
  }));
  v.src = getVideoURI("/media/movie_5") + "?" + new Date() + Math.random();
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
  "source_name": "html/semantics/embedded-content/media-elements/event_pause.html"
}
```
