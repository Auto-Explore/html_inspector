# html/semantics/embedded-content/media-elements/media_fragment_seek.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/media-elements/media_fragment_seek.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset="utf-8">
<meta name="timeout" content="long">
<title>Video should seek to time specified in media fragment syntax</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/common/media.js"></script>
<div id="log"></div>
<video id="video"></video>
<script>
async_test(function () {
  let video = document.getElementById("video");
  video.src = getVideoURI('/media/movie_5') + "#t=4,7";
  video.load();
  this.step_timeout(function () {
    assert_equals(video.currentTime, 4.0);

    video.src = getVideoURI('/media/movie_5') + "#t=%6Ept:3";
    video.load();
    this.step_timeout(function () {
      assert_true(video.src.endsWith("t=%6Ept:3"));
      assert_equals(video.currentTime, 3.0);

      video.src = getVideoURI('/media/movie_5') + "#t=00:00:01.00";
      video.load();
      this.step_timeout(function () {
        assert_true(video.src.endsWith("t=00:00:01.00"));
        assert_equals(video.currentTime, 1.0);

        video.src = getVideoURI('/media/movie_5') + "#u=12&t=3";
        video.load();
        this.step_timeout(function () {
          assert_true(video.src.endsWith("#u=12&t=3"));
          assert_equals(video.currentTime, 3.0);

          video.src = getVideoURI('/media/movie_5') + "#t=npt%3A3";
          video.load();
          this.step_timeout(function () {
            assert_true(video.src.endsWith("t=npt%3A3"));
            assert_equals(video.currentTime, 3.0);
            this.done();
          }, 1000);
        }, 1000);
      }, 1000);
    }, 1000);
  }, 1000);
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
  "source_name": "html/semantics/embedded-content/media-elements/media_fragment_seek.html"
}
```
