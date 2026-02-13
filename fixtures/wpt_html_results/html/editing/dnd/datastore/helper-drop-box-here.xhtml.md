# html/editing/dnd/datastore/helper-drop-box-here.xhtml

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/editing/dnd/datastore/helper-drop-box-here.xhtml",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<?xml version="1.0" encoding="utf-8"?>
<html xmlns="http://www.w3.org/1999/xhtml">
<head>
<title>Drag and drop datastore: helper file</title>
<style type="text/css">
html, body
  {height:100%;}
</style>
<script type="application/ecmascript">
<![CDATA[
var dataTypes = ['text/plain', 'text/uri-list', 'application/xml', 'application/xhtml+xml', 'application/mathml+xml', 'image/svg+xml', 'text/html', 'text/x-example'],
data = ['PASS', 'data:text/plain,1', '<result>PASS</result>', '<html xmlns="http://www.w3.org/1999/xhtml"><head><title>Data store item</title></head><body><p>PASS</p></body></html>', '<math xmlns="http://www.w3.org/1998/Math/MathML"><mn>1</mn></math>', '<svg xmlns="http://www.w3.org/2000/svg" version="1.1" width="100px" height="50px" viewBox="0 0 100 50"><text x="0" y="40" font-size="40" fill="green">PASS</text></svg>', '<!DOCTYPE html><html><head><title>Data store item</title></head><body><p>PASS</p></body></html>', 'PASS'];
function enterElement(event)
  {event.preventDefault();
  for(var i = 0; i != dataTypes.length; i++)
    {event.dataTransfer.setData(dataTypes[i], 'FAIL');
    if(event.dataTransfer.getData(dataTypes[i]))
      {say('getData(' + dataTypes[i] + ') : FAIL (data store should not be readable during dragenter)')}
    }
  say('items.length (dragenter) : ' + ((event.dataTransfer.items.length >= dataTypes.length)?'PASS':'FAIL'));}
function dataDrop(event)
  {say('items.length (drop) : ' + ((event.dataTransfer.items.length >= dataTypes.length)?'PASS':'FAIL'));
  for(var i = 0; i != dataTypes.length; i++)
    {say('getData(' + dataTypes[i] + ') : ' + ((event.dataTransfer.getData(dataTypes[i]) == data[i])?'PASS':'FAIL'));}
  document.querySelector('body').setAttribute('style','background-color:teal;color:white;');}
function say(it)
  {document.querySelector('pre').appendChild(document.createTextNode(it + '\n'));}
]]>
</script>
</head>
<body ondragenter="enterElement(event)" ondragover="return false;" ondrop="dataDrop(event)">
<p>Drop box here. Frame should turn green and test results should appear below.</p>
<pre/>
</body>
</html>
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
        "byte_end": 165,
        "byte_start": 142,
        "col": 1,
        "line": 5
      }
    },
    {
      "category": "Html",
      "code": "html.script.type.unnecessary",
      "message": "The “type” attribute is unnecessary for JavaScript resources.",
      "severity": "Warning",
      "span": {
        "byte_end": 241,
        "byte_start": 203,
        "col": 1,
        "line": 9
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
  "source_name": "html/editing/dnd/datastore/helper-drop-box-here.xhtml"
}
```
