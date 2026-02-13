# html/editing/dnd/file/010-manual.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/editing/dnd/file/010-manual.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>drag &amp; drop - blocked drop</title>
<style>
  body > div {
    height: 200px;
    width: 200px;
    background-color: orange;
  }
</style>

<script>
//This test assumes that if the page does not want to use the drop, that the browser will revert to default behaviour of opening the file
window.onload = function() {
  var orange = document.getElementsByTagName('div')[0];
  orange.ondragover = orange.ondragenter = function(e) {
    e.preventDefault();
    e.dataTransfer.dropEffect = 'none';
  };
  orange.ondrop = function(e) {
    e.preventDefault();
  };

};
</script>

<div></div>

<p>Save <a href="../resources/pass.png">this image</a> to your desktop. Use your pointing device to drag the saved file from your desktop onto the orange box, and release it. Fail if nothing happens.</p>
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
  "source_name": "html/editing/dnd/file/010-manual.html"
}
```
