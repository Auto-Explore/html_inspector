# html/editing/dnd/platform/file-drop-position.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/editing/dnd/platform/file-drop-position.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>drag &amp; drop - mouse coordinates during drop</title>
<style>
  body > div {
    height: 5px;
    width: 5px;
    background-color: orange;
  }
</style>

<script>
window.onload = function() {
  var orange = document.getElementsByTagName('div')[0];
  orange.ondragover = orange.ondragenter = orange.ondrop = function(e) {
    e.preventDefault();
  };
};
</script>

<div></div>

<p>Save <a href="../resources/fail.png">this image</a> to your desktop. Minimise your browser. Use your pointing device to drag the saved file from your desktop <strong>via your browser's button on your operating system's taskbar</strong> (so that it maximises your browser), onto the small orange box above this text, and release it. If a confirmation dialog appears, accept it. Fail if the browser simply displays the image.</p>
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
  "source_name": "html/editing/dnd/platform/file-drop-position.html"
}
```
