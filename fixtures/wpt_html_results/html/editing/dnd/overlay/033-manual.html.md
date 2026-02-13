# html/editing/dnd/overlay/033-manual.html

Counts:
- errors: 0
- warnings: 6
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/editing/dnd/overlay/033-manual.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<?xml version="1.0" encoding="utf-8"?>
<html xmlns="http://www.w3.org/1999/xhtml">
<head>
<title>Drag and drop of overlapping images: fixed position</title>
<style type="text/css">
span, strong
  {color:green;
  background-color:yellow;}
strong
  {color:red;}
div
  {height:100px;
  position:relative;}
img
  {display:block;
  position:fixed;
  top:0;
  bottom:0;
  width:100px;
  height:100px;}
</style>
<script type="application/ecmascript">
function start(event,feedback)
  {event.dataTransfer.effectAllowed = 'copy';
  event.dataTransfer.addElement(document.querySelector(feedback));}
</script>
</head>
<body>
<div>
  <img ondragstart="start(event,'strong')" src="data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mP4z8DwHwAFAAH/VscvDQAAAABJRU5ErkJggg=="/>
  <img ondragstart="start(event,'span')" src="data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mNgaGD4DwAChAGAJVtEDQAAAABJRU5ErkJggg=="/>
</div>
<p>Try to drag box above. You should see word <span>PASS</span> not <strong>FAIL</strong> in feedback overlay all the time.</p>
</body>
</html>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.tokenizer.processing_instruction",
      "message": "Saw “<?”. Probable cause: Attempt to use an XML processing instruction in HTML. (XML processing instructions are not supported in HTML.)",
      "severity": "Warning",
      "span": {
        "byte_end": 18,
        "byte_start": 16,
        "col": 1,
        "line": 2
      }
    },
    {
      "category": "Html",
      "code": "html.style.type.unnecessary",
      "message": "The “type” attribute for the “style” element is not needed and should be omitted.",
      "severity": "Warning",
      "span": {
        "byte_end": 196,
        "byte_start": 173,
        "col": 1,
        "line": 6
      }
    },
    {
      "category": "Html",
      "code": "html.script.type.unnecessary",
      "message": "The “type” attribute is unnecessary for JavaScript resources.",
      "severity": "Warning",
      "span": {
        "byte_end": 459,
        "byte_start": 421,
        "col": 1,
        "line": 23
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 805,
        "byte_start": 638,
        "col": 3,
        "line": 31
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 973,
        "byte_start": 808,
        "col": 3,
        "line": 32
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
  "source_name": "html/editing/dnd/overlay/033-manual.html"
}
```
