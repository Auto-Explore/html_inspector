# html/editing/dnd/roundtrip/008-manual.html

Counts:
- errors: 2
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/editing/dnd/roundtrip/008-manual.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<?xml version="1.0" encoding="utf-8"?>
<html xmlns="http://www.w3.org/1999/xhtml">
<head>
<title>Drag and drop roundtrip with text/uri-list data</title>
<style type="text/css">
div[ondragenter]
  {width:40px;
  min-height:40px;
  margin-top:20px;
  padding:40px;
  color:white;
  background-color:navy;}
</style>
</head>
<body>
<div
  draggable="true"
  ondragstart="event.dataTransfer.effectAllowed = 'copy';event.dataTransfer.setData('text/uri-list','data:text/plain,PASS')"
  ondragenter="event.preventDefault()"
  ondragover="return false"
  ondrop="document.querySelector('div').appendChild(document.createTextNode(event.dataTransfer.getData('text/uri-list').substr(16,4) + ' '))"
/>
<p>Drag blue box outside browser window and then drag it back and drop on itself. You should see word PASS once you drop it.</p>
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
        "byte_end": 2,
        "byte_start": 0,
        "col": 1,
        "line": 1
      }
    },
    {
      "category": "Html",
      "code": "html.parser.start_tag_without_doctype",
      "message": "Start tag seen without seeing a doctype first. Expected “<!DOCTYPE html>”.",
      "severity": "Error",
      "span": {
        "byte_end": 82,
        "byte_start": 39,
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
        "byte_end": 176,
        "byte_start": 153,
        "col": 1,
        "line": 5
      }
    },
    {
      "category": "Html",
      "code": "html.parse.self_closing.non_void",
      "message": "Self-closing syntax (“/>”) used on a non-void HTML element. Ignoring the slash and treating as a start tag.",
      "severity": "Error",
      "span": {
        "byte_end": 688,
        "byte_start": 328,
        "col": 1,
        "line": 16
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
  "source_name": "html/editing/dnd/roundtrip/008-manual.html"
}
```
