# html/rendering/non-replaced-elements/the-frameset-and-frame-elements/large-rows-abssize.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/the-frameset-and-frame-elements/large-rows-abssize.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel="author" title="Morten Stenshorne" href="mailto:mstensho@chromium.org">
<link rel="help" href="https://html.spec.whatwg.org/multipage/rendering.html#frames-and-framesets">
<link rel="help" href="https://bugs.chromium.org/p/chromium/issues/detail?id=1116832">
<link rel="match" href="reference/green-ref.html">
<frameset rows="4294967227%,*" frameborder="0">
  <frame src="resources/green.html">
  <frame src="resources/red.html">
</frameset>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.element.frameset.obsolete",
      "message": "The “frameset” element is obsolete. Use the “iframe” element and CSS instead, or use server-side includes.",
      "severity": "Warning",
      "span": {
        "byte_end": 383,
        "byte_start": 336,
        "col": 1,
        "line": 6
      }
    },
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
  "source_name": "html/rendering/non-replaced-elements/the-frameset-and-frame-elements/large-rows-abssize.html"
}
```
