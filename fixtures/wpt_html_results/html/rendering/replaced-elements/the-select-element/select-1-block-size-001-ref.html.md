# html/rendering/replaced-elements/the-select-element/select-1-block-size-001-ref.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/replaced-elements/the-select-element/select-1-block-size-001-ref.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>CSS Test Reference</title>
<link rel="author" title="Emilio Cobos Álvarez" href="mailto:emilio@crisal.io">
<link rel="author" title="Mozilla" href="https://mozilla.org">
<link rel="help" href="https://bugzilla.mozilla.org/show_bug.cgi?id=1571764">
<link rel="match" href="select-1-block-size-001-ref-2.html">
<style>
button {
  -webkit-appearance: none;
  appearance: none;

  background: black;
  color: transparent;

  line-height: 100px;
  width: 100px;

  border: 0;
  border-radius: 0;
  padding: 0;
}
</style>
<button>A</button>
<button>A</button>
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
  "source_name": "html/rendering/replaced-elements/the-select-element/select-1-block-size-001-ref.html"
}
```
