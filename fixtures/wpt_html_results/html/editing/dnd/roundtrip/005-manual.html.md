# html/editing/dnd/roundtrip/005-manual.html

Counts:
- errors: 1
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/editing/dnd/roundtrip/005-manual.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<?xml version="1.0" encoding="utf-8"?>
<html xmlns="http://www.w3.org/1999/xhtml">
<head>
<title>Selection drag and drop roundtrip</title>
</head>
<body onload="window.getSelection().selectAllChildren(document.querySelector('span'))">
<p><span ondragenter="event.preventDefault()" ondragover="return false" ondrop="document.querySelector('span').firstChild.nodeValue = (event.dataTransfer.getData('text/plain') == 'Drag me')?'PASS':'FAIL'">Drag me</span></p>
<p>Drag selected text outside browser window and then drag it back and drop on itself. You should see word PASS once you drop it.</p>
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
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/editing/dnd/roundtrip/005-manual.html"
}
```
