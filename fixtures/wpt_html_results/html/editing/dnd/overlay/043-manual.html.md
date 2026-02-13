# html/editing/dnd/overlay/043-manual.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/editing/dnd/overlay/043-manual.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>drag &amp; drop - dragging nested draggable elements</title>
<style>
  body > div {
    height: 200px;
    width: 200px;
    background-color: navy;
    position: absolute;
    top: 8px;
    left: 8px;
  }
  body > div div {
    height: 100px;
    width: 100px;
    background-color: orange;
    position: absolute;
    left: 50px;
    top: 50px;
  }
  body > div + div {
    background-color: fuchsia;
    height: 100px;
    width: 100px;
    left: 258px;
    top: 58px;
  }
  p:first-of-type {
    margin-top: 220px;
  }
</style>

<script>

window.onload = function() {
  var passed = true, orange = document.getElementsByTagName('div')[1], blue = document.getElementsByTagName('div')[0], fuchsia = document.getElementsByTagName('div')[2];

  orange.ondragstart = function(e) {
    e.dataTransfer.effectAllowed = 'copy';
    e.dataTransfer.setData('text/plain', 'child targeted');
  };
  blue.ondragstart = function(e) {
    if( e.target == this ) {
      e.dataTransfer.effectAllowed = 'copy';
      e.dataTransfer.setData('text/plain', 'parent targeted');
    } else {
      e.dataTransfer.setData('extra/data', 'parent bubble');
    }
  };
  fuchsia.ondragenter = fuchsia.ondragover = function(e) {
    e.preventDefault();
    e.dataTransfer.dropEffect = 'copy';
  };
  fuchsia.ondrop = function(e) {
    e.preventDefault();
    //it's possible this could get called twice if the browser drags both items, so it uses the "passed" variable to make sure
    //that if blue gets dropped first, it remains false when orange then gets dropped
    passed = passed && ( e.dataTransfer.getData('text/plain') == 'child targeted' ) && ( e.dataTransfer.getData('extra/data') == 'parent bubble' );
    document.getElementsByTagName('p')[0].innerHTML = passed ? 'PASS' : 'FAIL';
  };

};

</script>

<div draggable='true'><div draggable='true'></div></div><div></div>

<p>Use your pointing device to drag the orange box to the pink box, then release it. While dragging, the drag placeholder should show that only the orange box is being dragged.</p>
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
  "source_name": "html/editing/dnd/overlay/043-manual.html"
}
```
