# html/semantics/embedded-content/the-video-element/video_dynamic_poster_relative.htm

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-video-element/video_dynamic_poster_relative.htm",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE HTML>
<meta charset=UTF-8>
<title>The 'HTMLVideoElement' interface supports setting 'poster' to a relative URL</title>
<link rel="author" title="Microsoft" href="http://www.microsoft.com/">
<link rel="help" href="https://html.spec.whatwg.org/multipage/#dom-video-poster">
<link rel="match" href="video_dynamic_poster-ref.htm">
<meta name="assert" content="The 'HTMLVideoElement' interface supports setting 'poster' to a relative URL">
<video id="video0">Your browser does not support video.</video>
<script>
var testElem = document.getElementById("video0");
testElem.poster = "../../../../media/poster.png";
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
  "source_name": "html/semantics/embedded-content/the-video-element/video_dynamic_poster_relative.htm"
}
```
