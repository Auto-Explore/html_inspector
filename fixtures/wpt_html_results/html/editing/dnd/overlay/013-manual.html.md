# html/editing/dnd/overlay/013-manual.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/editing/dnd/overlay/013-manual.html",
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
<title>Feedback image and CSS transforms</title>
<style type="text/css">
body
  {background-color:silver;
  margin:0;}
div
  {background-color:white;}
div > div
  {display:block;
  width:100px;
  height:87px;
  transform-origin:bottom right;
  transform:skew(-30deg);
  -o-transform-origin:bottom right;
  -o-transform:skew(-30deg);
  background-color:green;}
</style>
</head>
<body>
<div>
  <div draggable="true" ondragstart="event.dataTransfer.effectAllowed = 'copy'"></div>
</div>
<p>Try to drag green rhomb above. Feedback overlay should not be rectangular.</p>
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
        "byte_end": 178,
        "byte_start": 155,
        "col": 1,
        "line": 6
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
  "source_name": "html/editing/dnd/overlay/013-manual.html"
}
```
