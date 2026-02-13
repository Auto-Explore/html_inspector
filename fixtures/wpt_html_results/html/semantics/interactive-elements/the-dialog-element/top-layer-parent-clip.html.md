# html/semantics/interactive-elements/the-dialog-element/top-layer-parent-clip.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/interactive-elements/the-dialog-element/top-layer-parent-clip.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
<title>Test that parent clip-path does not affect top layer elements</title>
<meta charset="utf-8">
<link rel="author" title="Tim Nguyen" href="https://github.com/nt1m">
<link rel="match" href="green-dialog-and-backdrop.html">
<link rel="help" href="https://fullscreen.spec.whatwg.org/#new-stacking-layer">
<style>
body { background: red; }

#parent {
    clip-path: circle(5%);
}

dialog::backdrop,
dialog {
    background: green;
    outline: none;
}
</style>
<body>
<div id="parent">
    <dialog>PASS if no red shows</dialog>
</div>
<script>
    document.querySelector("dialog").showModal();
</script>
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
  "source_name": "html/semantics/interactive-elements/the-dialog-element/top-layer-parent-clip.html"
}
```
