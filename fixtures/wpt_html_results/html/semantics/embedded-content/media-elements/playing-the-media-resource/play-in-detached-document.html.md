# html/semantics/embedded-content/media-elements/playing-the-media-resource/play-in-detached-document.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/media-elements/playing-the-media-resource/play-in-detached-document.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>play() in detached document</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/common/media.js"></script>
<div id="log"></div>
<script>
// Negative test for failure to play in a detached document.
async_test(function(t)
{
  var doc = document.implementation.createHTMLDocument("");
  var v = doc.createElement("video");
  doc.body.appendChild(v);
  v.src = getVideoURI("/media/movie_5");
  v.play().catch(() => {});

  v.addEventListener("timeupdate", t.step_func(function() {
    assert_false(v.paused);
    if (v.currentTime > 0) {
      t.done();
    }
  }));
});
</script>
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
  "source_name": "html/semantics/embedded-content/media-elements/playing-the-media-resource/play-in-detached-document.html"
}
```
