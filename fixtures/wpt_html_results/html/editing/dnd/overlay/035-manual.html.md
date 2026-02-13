# html/editing/dnd/overlay/035-manual.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/editing/dnd/overlay/035-manual.html",
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
<title>Drag and drop of floated overlapping elements: negative margins</title>
<style type="text/css">
div
  {height:100px;
  width:100px;
  float:left;
  margin-left:-50px;
  background-color:navy;}
div:nth-child(odd)
  {background-color:maroon;
  margin-top:50px;}
div[draggable]
  {background-color:teal;}
</style>
</head>
<body>
<p>Only green areas should be draggable.</p>
<div></div>
<div></div>
<div draggable="true"></div>
<div></div>
<div></div>
<div draggable="true"></div>
<div></div>
<div></div>
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
        "byte_end": 208,
        "byte_start": 185,
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
  "source_name": "html/editing/dnd/overlay/035-manual.html"
}
```
