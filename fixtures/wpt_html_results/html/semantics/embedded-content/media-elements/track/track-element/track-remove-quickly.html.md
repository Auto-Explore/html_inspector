# html/semantics/embedded-content/media-elements/track/track-element/track-remove-quickly.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/media-elements/track/track-element/track-remove-quickly.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Removing a track element before it has been processed doesn't crash</title>
<script src="/common/media.js"></script>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id="video_container"></div>
<script>
var mediaFile = getVideoURI("/media/test");
document.getElementById("video_container").innerHTML = "<video src='" + mediaFile + "' controls ><track kind='captions' src='resources/simple-captions.vtt' default ></video>";
test(function() {
// https://bugs.webkit.org/show_bug.cgi?id=85095
// Test passes if it doesn't crash.
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
  "source_name": "html/semantics/embedded-content/media-elements/track/track-element/track-remove-quickly.html"
}
```
