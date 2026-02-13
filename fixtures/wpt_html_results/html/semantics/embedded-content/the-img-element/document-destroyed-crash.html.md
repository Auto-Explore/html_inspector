# html/semantics/embedded-content/the-img-element/document-destroyed-crash.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-img-element/document-destroyed-crash.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>&lt;img> loading in destroyed document</title>
<iframe></iframe>
<script>
onload = function() {
  const img = new Image();
  img.onload = function() {
    const iframe = document.querySelector('iframe');
    iframe.contentDocument.createElement('div').innerHTML =
      `<picture>
         <source srcset="nonexistent.png">
         <img src="data:image/gif;base64,R0lGODlhCgAKAIAAAP/MAAAAACH5BAAAAAAALAAAAAAKAAoAAAIIhI+py+0PYysAOw==">
       </picture>`;
    iframe.remove();
  };
  img.src = 'data:image/gif;base64,R0lGODlhCgAKAIAAAP/MAAAAACH5BAAAAAAALAAAAAAKAAoAAAIIhI+py+0PYysAOw==';
};
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
  "source_name": "html/semantics/embedded-content/the-img-element/document-destroyed-crash.html"
}
```
