# html/semantics/embedded-content/the-iframe-element/support/iframe_sandbox_002.htm

Counts:
- errors: 1
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-iframe-element/support/iframe_sandbox_002.htm",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
<head>
    <title>HTML5 video with autoplay attribute.</title>
    <script src="/common/media.js"></script>
</head>
<body>
    <script>
    function do_play(event) {
        parent.window.postMessage("play event fired", "*");
    }

    document.write(
        "<video id='video0' src='" + getVideoURI("/media/green-at-15") + "'" +
        " autoplay onplay='do_play(event);'>"
    );
    </script>
    Your browser does not support HTML5 video.
    </video>
</body>
</html>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.parser.stray_end_tag",
      "message": "Stray end tag “video”.",
      "severity": "Error",
      "span": {
        "byte_end": 481,
        "byte_start": 473,
        "col": 5,
        "line": 19
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
  "source_name": "html/semantics/embedded-content/the-iframe-element/support/iframe_sandbox_002.htm"
}
```
