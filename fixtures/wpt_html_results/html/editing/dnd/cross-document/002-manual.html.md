# html/editing/dnd/cross-document/002-manual.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/editing/dnd/cross-document/002-manual.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>drag &amp; drop - cross-domain cross-document data drop</title>
<script src="../resources/crossorigin.sub.js"></script>
<style>
  body > div {
    height: 200px;
    width: 200px;
    background-color: orange;
  }
</style>

<script>
window.onload = function() {
  var orange = document.getElementsByTagName('div')[0];
  orange.ondragstart = function(e) {
    e.dataTransfer.effectAllowed = 'copy';
    e.dataTransfer.setData('text', 'dummy text');
  };
};
</script>

<div draggable="true"></div>
<p><iframe height="300" width="500"></iframe></p>
<script>document.getElementsByTagName("iframe")[0].src = crossOriginUrl("www", "001-1.html");</script>
<noscript><p>Enable JavaScript and reload</p></noscript>
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
  "source_name": "html/editing/dnd/cross-document/002-manual.html"
}
```
