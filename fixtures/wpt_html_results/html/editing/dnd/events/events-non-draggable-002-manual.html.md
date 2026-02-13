# html/editing/dnd/events/events-non-draggable-002-manual.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/editing/dnd/events/events-non-draggable-002-manual.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset='utf-8'>
<title>drag &amp; drop – events should not fire with non-draggable elements – 002</title>
<style type="text/css">
  div {
    height: 200px;
    width: 200px;
    background-color: orange;
    position: absolute;
    top: 8px;
    left: 8px;
  }
  div + div {
    background-color: navy;
    left: 250px;
  }

  div + p {
    margin-top: 220px;
  }
</style>

<script>
window.onload = function() {

  var orange = document.getElementsByTagName('div')[0];
  var blue = document.getElementsByTagName('div')[1];
  var body = document.body;

  var pass = true;

  function fail() {
    pass = false;
  }

  body.addEventListener('drag',fail,false);
  body.addEventListener('dragend',fail,false);
  body.addEventListener('dragenter',fail,false);
  body.addEventListener('dragleave',fail,false);
  body.addEventListener('dragover',fail,false);
  body.addEventListener('dragstart',fail,false);
  body.addEventListener('drop',fail,false);

  body.ondrag = fail;
  body.ondragend = fail;
  body.ondragenter = fail;
  body.ondragleave = fail;
  body.ondragover = fail;
  body.ondragstart = fail;
  body.ondrop = fail;

  orange.addEventListener('drag',fail,false);
  orange.addEventListener('dragend',fail,false);
  orange.addEventListener('dragenter',fail,false);
  orange.addEventListener('dragleave',fail,false);
  orange.addEventListener('dragover',fail,false);
  orange.addEventListener('dragstart',fail,false);
  orange.addEventListener('drop',fail,false);

  orange.ondrag = fail;
  orange.ondragend = fail;
  orange.ondragenter = fail;
  orange.ondragleave = fail;
  orange.ondragover = fail;
  orange.ondragstart = fail;
  orange.ondrop = fail;

  blue.addEventListener('drag',fail,false);
  blue.addEventListener('dragend',fail,false);
  blue.addEventListener('dragenter',fail,false);
  blue.addEventListener('dragleave',fail,false);
  blue.addEventListener('dragover',fail,false);
  blue.addEventListener('dragstart',fail,false);
  blue.addEventListener('drop',fail,false);

  blue.ondrag = fail;
  blue.ondragend = fail;
  blue.ondragenter = fail;
  blue.ondragleave = fail;
  blue.ondragover = fail;
  blue.ondragstart = fail;
  blue.ondrop = fail;

  body.onmouseup = function () {
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
<div></div>

<p>Use your pointing device to drag from the orange box to the blue box. The
word &quot;PASS&quot; should appear.
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
  "source_name": "html/editing/dnd/events/events-non-draggable-002-manual.html"
}
```
