# html/semantics/embedded-content/media-elements/video_controls_present-manual.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/media-elements/video_controls_present-manual.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
  <head>
    <title>Video Test: video_controls_present.html</title>
    <link rel="author" title="Intel" href="http://www.intel.com" />
    <link rel="help" href="https://html.spec.whatwg.org/multipage/#dom-media-controls" />
    <meta name="flags" content="" />
    <meta name="assert" content="Check if the controls attribute is present in the video element that expecting the user agent exposes a controller user interface" />
  </head>
  <body>
    <p>Test passes if a controller user interface appears below and the text 'The user agent doesn't support media element.' does not appear anywhere on this page</p>
    <video id="m" controls>The user agent doesn't support media element.</video>
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
  "source_name": "html/semantics/embedded-content/media-elements/video_controls_present-manual.html"
}
```
