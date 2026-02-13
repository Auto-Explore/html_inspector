# html/rendering/replaced-elements/embedded-content/video-controls-vertical-writing-mode.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/replaced-elements/embedded-content/video-controls-vertical-writing-mode.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset=utf-8>
<title>Video controls rendering in vertical-lr</title>
<link rel="help" href="https://html.spec.whatwg.org/multipage/rendering.html#embedded-content-rendering-rules" />
<link rel="match" href="video-controls-vertical-writing-mode-ref.html" />
<div style="writing-mode:vertical-lr">
  <video controls></video>
</div>
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
  "source_name": "html/rendering/replaced-elements/embedded-content/video-controls-vertical-writing-mode.html"
}
```
