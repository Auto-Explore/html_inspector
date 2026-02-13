# html/rendering/replaced-elements/embedded-content-rendering-rules/audio-controls-002.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/replaced-elements/embedded-content-rendering-rules/audio-controls-002.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>HTML audio with controls via Web APIs</title>
<link rel="author" title="Mozilla" href="https://www.mozilla.org/">
<link rel="help" href="https://html.spec.whatwg.org/multipage/rendering.html#embedded-content-rendering-rules">
<link rel="mismatch" href="/common/blank.html">

<audio id="target"></audio>

<script>
  document.body.offsetTop;
  document.getElementById("target").setAttribute("controls", "");
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
  "source_name": "html/rendering/replaced-elements/embedded-content-rendering-rules/audio-controls-002.html"
}
```
