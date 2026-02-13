# html/editing/dnd/overlay/005-manual.html

Counts:
- errors: 0
- warnings: 4
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/editing/dnd/overlay/005-manual.html",
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
<title>Adding element to datastore on dragenter</title>
<style type="text/css">
span, strong
  {color:green;
  background-color:yellow;}
strong
  {color:red;}
div
  {width:100px;
  height:100px;
  background-color:silver;
  margin-top:20px;}
</style>
<script type="application/ecmascript">
function start(event)
  {event.dataTransfer.effectAllowed = 'copy';
  event.dataTransfer.addElement(document.querySelector('span'));}
function resetImage(event)
  {event.preventDefault();
  event.dataTransfer.addElement(document.querySelector('strong'));}
</script>
</head>
<body>
<p><a href="data:text/plain,1" ondragstart="start(event)">Drag me</a></p>
<p>Try to drag link above to the silver box. You should see word <span>PASS</span> not <strong>FAIL</strong> in feedback overlay all the time.</p>
<div ondragenter="resetImage(event)" ondragover="return false"></div>
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
        "byte_end": 185,
        "byte_start": 162,
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
        "byte_end": 395,
        "byte_start": 357,
        "col": 1,
        "line": 18
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
  "source_name": "html/editing/dnd/overlay/005-manual.html"
}
```
