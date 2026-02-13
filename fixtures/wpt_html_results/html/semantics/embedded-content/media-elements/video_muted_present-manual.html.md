# html/semantics/embedded-content/media-elements/video_muted_present-manual.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/media-elements/video_muted_present-manual.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
  <head>
    <title>Video Test: video_muted_present</title>
    <link rel="author" title="Intel" href="http://www.intel.com" />
    <link rel="help" href="https://html.spec.whatwg.org/multipage/#dom-media-muted" />
    <meta name="flags" content="" />
    <meta name="assert" content="Check if the muted attribute is present in the video element that expecting the user hears no sound" />
    <script src="/common/media.js"></script>
  </head>
  <body>
    <p>Test passes if the video is playing without sound output and the text 'The user agent doesn't support media element.' does not appear anywhere on this page</p>
    <video id="m" controls muted>The user agent doesn't support media element.</video>
    <script type="text/javascript">
        var media = document.getElementById("m");
        media.src = getVideoURI("/media/movie_5") + "?" + new Date() + Math.random();
        media.play();
    </script>
  </body>
</html>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.script.type.unnecessary",
      "message": "The “type” attribute is unnecessary for JavaScript resources.",
      "severity": "Warning",
      "span": {
        "byte_end": 765,
        "byte_start": 734,
        "col": 5,
        "line": 14
      }
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/semantics/embedded-content/media-elements/video_muted_present-manual.html"
}
```
