# html/editing/dnd/overlay/024-manual.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/editing/dnd/overlay/024-manual.html",
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
<title>PNG image drag and drop: changing draggable attribute</title>
<style type="text/css">
img
  {width:100px;
  height:100px;}
</style>
</head>
<body onload="document.querySelector('img').setAttribute('draggable','false')">
<p><img ondragstart="document.querySelector('p').firstChild.nodeValue = 'FAIL'" src="data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAIAAAACCAYAAABytg0kAAAAEElEQVR42mNgaGD4D8YwBgAw9AX9Y9zBwwAAAABJRU5ErkJggg==" alt="PNG green pixel"/></p>
<p>You should not be able to drag green box.</p>
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
        "byte_end": 198,
        "byte_start": 175,
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
  "source_name": "html/editing/dnd/overlay/024-manual.html"
}
```
