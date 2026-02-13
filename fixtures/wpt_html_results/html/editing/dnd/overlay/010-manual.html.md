# html/editing/dnd/overlay/010-manual.html

Counts:
- errors: 0
- warnings: 4
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/editing/dnd/overlay/010-manual.html",
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
<title>Resetting drag image on new drag</title>
<style type="text/css">
span
  {color:green;
  background-color:yellow;}
</style>
<script type="application/ecmascript">
var i = 0;
function start(event)
  {event.dataTransfer.effectAllowed = 'copy';
  event.dataTransfer.setDragImage(document.querySelectorAll('span')[i++%2], 1, 1);}
</script>
</head>
<body>
<p><a href="data:text/plain,1" ondragstart="start(event)">Drag me</a></p>
<p>Drag link above around the page drop it and try to drag again.</p>
<p>First time you drag it you should see word <span>Odd</span> in feedback overlay, second time overlay should change to <span>Even</span>.</p>
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
        "byte_end": 177,
        "byte_start": 154,
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
        "byte_end": 274,
        "byte_start": 236,
        "col": 1,
        "line": 11
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
  "source_name": "html/editing/dnd/overlay/010-manual.html"
}
```
