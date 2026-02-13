# html/editing/dnd/overlay/044-manual.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/editing/dnd/overlay/044-manual.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>drag &amp; drop - dragging selections inside draggable elements</title>
<style>
  body > div {
    height: 200px;
    width: 200px;
    background-color: orange;
    position: absolute;
    top: 8px;
    left: 8px;
  }
  body > div + div {
    background-color: fuchsia;
    left: 250px;
  }
  p:first-of-type {
    margin-top: 220px;
  }
</style>

<script>

window.onload = function() {
  var orange = document.getElementsByTagName('div')[0], fuchsia = document.getElementsByTagName('div')[1];

  orange.ondragstart = function(e) {
    e.dataTransfer.effectAllowed = 'copy';
    e.dataTransfer.setData('extra/data', 'parent bubble');
  };
  fuchsia.ondragenter = fuchsia.ondragover = function(e) {
    e.preventDefault();
    e.dataTransfer.dropEffect = 'copy';
  };
  fuchsia.ondrop = function(e) {
    e.preventDefault();
    var passed = ( e.dataTransfer.getData('text/plain') == 'text dummy' ) && ( e.dataTransfer.getData('extra/data') == 'parent bubble' );
    document.getElementsByTagName('p')[0].innerHTML = passed ? 'PASS' : 'FAIL';
  };
  var range = document.createRange();
  range.selectNodeContents(orange);
  range.setStart(orange.firstChild,6);
  range.setEnd(orange.firstChild,16);
  window.getSelection().addRange(range);

};

</script>

<div draggable='true'>Dummy text dummy text</div><div></div>

<p>Use your pointing device to <strong>drag the selected text</strong> to the pink box, then release it. While dragging, the drag placeholder should show that only the selected text is being dragged.</p>
<p>(If no text is selected, you will need to use your browser's functionality to select &quot;text dummy&quot; in the orange box.)</p>
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
  "source_name": "html/editing/dnd/overlay/044-manual.html"
}
```
