# html/semantics/embedded-content/media-elements/event_timeupdate.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/media-elements/event_timeupdate.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<html>
 <head>
  <title>{audio,video} events - timeupdate</title>
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
var ta = async_test("setting src attribute on a sufficiently long autoplay audio should trigger timeupdate event");
var a = document.getElementById("a");
a.addEventListener("timeupdate", function() {
  ta.done();
  a.pause();
}, false);
a.src = getAudioURI("/media/sound_5") + "?" + new Date() + Math.random();

var tv = async_test("setting src attribute on a sufficiently long autoplay video should trigger timeupdate event");
var v = document.getElementById("v");
v.addEventListener("timeupdate", function() {
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
  "source_name": "html/semantics/embedded-content/media-elements/event_timeupdate.html"
}
```
