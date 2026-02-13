# html/editing/dnd/overlay/030-manual.html

Counts:
- errors: 0
- warnings: 4
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/editing/dnd/overlay/030-manual.html",
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
<title>Drag and drop of overlapping links: fixed position</title>
<style type="text/css">
span, strong
  {color:green;
  background-color:yellow;}
strong
  {color:red;}
div
  {height:2em;
  position:fixed;
  top:10px;
  left:10px;}
div + div
  {z-index:2;}
p
  {margin-top:3em;}
</style>
<script type="application/ecmascript">
function start(event,feedback)
  {event.dataTransfer.effectAllowed = 'copy';
  event.dataTransfer.addElement(document.querySelector(feedback));}
</script>
</head>
<body>
<div>
  <a href="data:text/plain,1" ondragstart="start(event,'strong')">   </a>
</div>
<div>
  <a href="data:text/plain,2" ondragstart="start(event,'span')">Link</a>
</div>
<p>Try to drag link above. You should see word <span>PASS</span> not <strong>FAIL</strong> in feedback overlay all the time.</p>
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
        "byte_end": 195,
        "byte_start": 172,
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
        "byte_end": 432,
        "byte_start": 394,
        "col": 1,
        "line": 22
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
  "source_name": "html/editing/dnd/overlay/030-manual.html"
}
```
