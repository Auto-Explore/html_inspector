# html/rendering/replaced-elements/embedded-content-rendering-rules/canvas-fallback.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/replaced-elements/embedded-content-rendering-rules/canvas-fallback.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset=utf-8>
<title>Canvas fallback content</title>
<link rel=match href=canvas-fallback-ref.html>
<style>
canvas {
  height: 2em; /* avoid causing scrollbar for 600x600 viewport */
}

#canvas2 {
  display: inline;
}

#canvas3 {
  display: block;
}

#canvas4 {
  display: table;
}
</style>
<p>The word "FAIL" should not be visible below this line.
<p><canvas id=canvas1>FAIL</canvas>
<p><canvas id=canvas2>FAIL</canvas>
<p><canvas id=canvas3>FAIL</canvas>
<p><canvas id=canvas4>FAIL</canvas>
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
  "source_name": "html/rendering/replaced-elements/embedded-content-rendering-rules/canvas-fallback.html"
}
```
