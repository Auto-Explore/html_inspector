# html/rendering/replaced-elements/embedded-content-rendering-rules/audio-without-controls.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/replaced-elements/embedded-content-rendering-rules/audio-without-controls.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>HTML audio without controls</title>
<link rel="author" title="Mozilla" href="https://www.mozilla.org/">
<link rel="help" href="https://html.spec.whatwg.org/multipage/rendering.html#embedded-content-rendering-rules">
<link rel="match" href="../../../../css/reference/ref-filled-green-100px-square.xht" />

<p>Test passes if there is a filled green square and <strong>no red</strong>.</p>

<div style="width: 100px; height: 100px; background: green;">
  <audio style="display: block; width: 100px; height: 100px; background: red;"></audio>
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
  "source_name": "html/rendering/replaced-elements/embedded-content-rendering-rules/audio-without-controls.html"
}
```
