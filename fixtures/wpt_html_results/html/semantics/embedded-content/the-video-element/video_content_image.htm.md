# html/semantics/embedded-content/the-video-element/video_content_image.htm

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-video-element/video_content_image.htm",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE HTML>
<html>
  <head>
    <title>HTML5 Media Elements: Content inside the 'video' element is not shown to the user.</title>
    <meta content="text/html; charset=UTF-8" http-equiv="Content-Type">
    <link rel="author" title="Microsoft" href="http://www.microsoft.com/" />
    <link rel="help" href="https://html.spec.whatwg.org/multipage/#video" />
    <link rel="match" href="video_content-ref.htm" />
    <meta name="assert" content="Content inside the 'video' element is not shown to the user (image)." />
  </head>
  <body>
  <div id='testcontent'>
    <video><img src="../../../../images/fail.gif" /></video>
  </div>
</body>
</html>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 616,
        "byte_start": 575,
        "col": 12,
        "line": 13
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
  "source_name": "html/semantics/embedded-content/the-video-element/video_content_image.htm"
}
```
