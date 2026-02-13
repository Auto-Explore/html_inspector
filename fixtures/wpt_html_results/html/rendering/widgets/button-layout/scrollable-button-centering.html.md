# html/rendering/widgets/button-layout/scrollable-button-centering.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/widgets/button-layout/scrollable-button-centering.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset="utf-8">
<title>Scrollable buttons are center-aligned and derive a baseline from their contents</title>
<link rel="author" href="mailto:emilio@crisal.io" title="Emilio Cobos Álvarez">
<link rel="author" href="https://mozilla.com" title="Mozilla">
<link rel="help" href="https://html.spec.whatwg.org/#button-layout">
<link rel="help" href="https://bugzilla.mozilla.org/show_bug.cgi?id=1977669">
<link rel="match" href="scrollable-button-centering-ref.html">
<style>
  button {
    height: 5em;
  }
</style>
<button style="overflow: hidden">ABC</button>
<button>ABC</button>
<button style="overflow: auto">ABC</button>
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
  "source_name": "html/rendering/widgets/button-layout/scrollable-button-centering.html"
}
```
