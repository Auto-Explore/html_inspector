# html/semantics/embedded-content/the-audio-element/audio_002.htm

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-audio-element/audio_002.htm",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE HTML>
<html>
<head>
  <title>HTML5 Media Elements: Content inside the 'audio' element is not shown to the user.</title>
  <meta content="text/html; charset=UTF-8" http-equiv="Content-Type">
  <link rel="author" title="Microsoft" href="http://www.microsoft.com/" />
  <link rel="help" href="https://html.spec.whatwg.org/multipage/#audio" />
  <link rel="match" href="audio_content-ref.htm" />
  <meta name="assert" content="Content inside the 'audio' element is not shown to the user." />
</head>
<body>
<p>Test passes if there is no red.</p>
<div id='testcontent'>
<audio><span style="color: red;">FAIL</span></audio>

</div>
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
  "source_name": "html/semantics/embedded-content/the-audio-element/audio_002.htm"
}
```
