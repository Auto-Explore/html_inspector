# html/rendering/non-replaced-elements/the-frameset-and-frame-elements/frame-no-frameset-crash.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/the-frameset-and-frame-elements/frame-no-frameset-crash.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<link rel=help href="https://bugzilla.mozilla.org/show_bug.cgi?id=1917361">
<script>
document.addEventListener("DOMContentLoaded", () => {
  const a = new Document()
  const b = a.createElementNS("http://www.w3.org/1999/xhtml", "frame")
  document.documentElement.appendChild(b)
  try { b.contentWindow.history.replaceState(undefined, "𛵢") } catch (e) {}
  b.contentWindow.history.go()
})
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.head.title.missing",
      "message": "Element “head” is missing a required instance of child element “title”.",
      "severity": "Warning",
      "span": null
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/rendering/non-replaced-elements/the-frameset-and-frame-elements/frame-no-frameset-crash.html"
}
```
