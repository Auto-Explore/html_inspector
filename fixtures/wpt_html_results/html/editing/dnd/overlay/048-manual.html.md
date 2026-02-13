# html/editing/dnd/overlay/048-manual.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/editing/dnd/overlay/048-manual.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>drag &amp; drop - dragging objects inside draggable elements</title>
<style>
  body > div {
    height: 200px;
    width: 200px;
    background-color: blue;
    position: absolute;
    top: 8px;
    left: 8px;
  }
  body > div + div {
    background-color: fuchsia;
    left: 258px;
  }
  object {
    border: 5px solid yellow;
    height: 130px;
    width: 130px;
    position: absolute;
    top: 30px;
    left: 30px;
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
    e.dataTransfer.setData('text/plain', 'FAIL');
  };
  fuchsia.ondragenter = fuchsia.ondragover = function(e) {
    e.preventDefault();
    e.dataTransfer.dropEffect = 'copy';
  };
  fuchsia.ondrop = function(e) {
    e.preventDefault();
    document.getElementsByTagName('p')[0].innerHTML = 'FAIL';
  };

};

</script>

<div draggable='true'><object data="about:blank"></object></div><div></div>

<p>Use your pointing device to begin dragging inside the yellow border (not on any scrollbars that may appear), over to the pink box, then release it. Pass if nothing is dragged, and if this text does not change.</p>
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
  "source_name": "html/editing/dnd/overlay/048-manual.html"
}
```
