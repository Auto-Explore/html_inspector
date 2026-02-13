# html/semantics/embedded-content/media-elements/src_object_blob.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/media-elements/src_object_blob.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset="utf-8">
<title>HTMLMediaElement.srcObject blob</title>
<script src='/common/media.js'></script>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src=/resources/testdriver.js></script>
<script src=/resources/testdriver-vendor.js></script>
<video></video>
<script>
  const video = document.querySelector("video");
  promise_test(async () => {
    const blob = await fetch(getVideoURI('/media/movie_5'))
          .then(r => r.blob());
    try {
      video.srcObject = blob;
    } catch (error) {
      assert_unreached(error);
    }
    const done = new Promise(res => video.addEventListener('ended', res));
    test_driver.bless('initiate media playback', function () {
      video.play();
    });
    return done;
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
  "source_name": "html/semantics/embedded-content/media-elements/src_object_blob.html"
}
```
