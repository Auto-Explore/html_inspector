# html/rendering/widgets/button-layout/scrollbars.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/widgets/button-layout/scrollbars.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset="utf-8">
<link rel="author" href="mailto:emilio@crisal.io" title="Emilio Cobos Álvarez">
<link rel="author" href="https://mozilla.org" title="Mozilla">
<link rel="help" href="https://bugzilla.mozilla.org/show_bug.cgi?id=1873301">
<link rel="help" href="https://html.spec.whatwg.org/#button-layout">
<link rel="match" href="/css/reference/ref-filled-green-100px-square-only.html">
<title>Buttons support being scrolled</title>
<style>
  button {
    display: block;
    width: 100px;
    height: 100px;
    background-color: red;
    border: 0;
    padding: 0;
    margin: 0;
    overflow: auto;
    scrollbar-width: none;
  }
  .filler {
    display: block;
    background: red;
    height: 400px;
  }
  .inner {
    display: block;
    width: 100px;
    height: 100px;
    background-color: green;
  }
</style>
<p>Test passes if there is a filled green square.</p>
<button>
  <span class="filler"></span>
  <span class="inner"></span>
</button>
<script>
  document.querySelector("button").scrollTop = 400;
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
  "source_name": "html/rendering/widgets/button-layout/scrollbars.html"
}
```
