# html/semantics/popovers/popover-alignment-002-ref.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/popovers/popover-alignment-002-ref.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Tests that popover alignment responds to anchor positioning</title>
<link rel="author" href="mailto:tabatkins@google.com">
<link rel="author" title="Elika J. Etemad" href="http://fantasai.inkedblade.net/contact">
<style>
button {
  border: solid blue 15px;
  margin: 25px;
  anchor-name: --foo;
}
div {
  position: absolute;
  border: solid orange 10px;
  inset: 0;
  margin: 0;
  padding: .25em;
  place-self: normal;
  position-anchor: --foo;
}
</style>

Orange box should be centered vertically against the left edge of the blue box.
<button></button>
<div style="position-area: left span-all"></div>
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
  "source_name": "html/semantics/popovers/popover-alignment-002-ref.html"
}
```
