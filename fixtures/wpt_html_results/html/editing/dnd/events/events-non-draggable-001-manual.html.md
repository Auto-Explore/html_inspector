# html/editing/dnd/events/events-non-draggable-001-manual.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/editing/dnd/events/events-non-draggable-001-manual.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset='utf-8'>
<title>drag &amp; drop – events should not fire with non-draggable elements – 001</title>
<style type="text/css">
  div {
    height: 200px;
    width: 200px;
    background-color: orange;
    position: absolute;
    top: 8px;
    left: 8px;
  }
  div + p {
    margin-top: 220px;
  }
</style>

<script>
window.onload = function() {

  var elem = document.getElementsByTagName('div')[0];
  var pass = true;

  function fail() {
    pass = false;
  }

  elem.addEventListener('drag',fail,false);
  elem.addEventListener('dragend',fail,false);
  elem.addEventListener('dragenter',fail,false);
  elem.addEventListener('dragleave',fail,false);
  elem.addEventListener('dragover',fail,false);
  elem.addEventListener('dragstart',fail,false);
  elem.addEventListener('drop',fail,false);

  elem.ondrag = fail;
  elem.ondragend = fail;
  elem.ondragenter = fail;
  elem.ondragleave = fail;
  elem.ondragover = fail;
  elem.ondragstart = fail;
  elem.ondrop = fail;

  elem.onmouseup = function () {
    setTimeout(function () {
      if (pass == true) {
        document.body.innerHTML = 'PASS';
      } else {
        document.body.innerHTML = 'FAIL';
      }
    }, 100 );
  };

}
</script>

<div></div>

<p>Click once on the orange box above, without moving the mouse while
clicking. The word &quot;PASS&quot; should appear.</p>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.style.type.unnecessary",
      "message": "The “type” attribute for the “style” element is not needed and should be omitted.",
      "severity": "Warning",
      "span": {
        "byte_end": 156,
        "byte_start": 133,
        "col": 1,
        "line": 4
      }
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/editing/dnd/events/events-non-draggable-001-manual.html"
}
```
