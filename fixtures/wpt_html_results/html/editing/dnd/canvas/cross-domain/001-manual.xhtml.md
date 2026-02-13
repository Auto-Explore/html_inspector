# html/editing/dnd/canvas/cross-domain/001-manual.xhtml

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/editing/dnd/canvas/cross-domain/001-manual.xhtml",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<?xml version="1.0" encoding="utf-8"?>
<html xmlns="http://www.w3.org/1999/xhtml">
<head>
<title>Cross-domain canvas data must not populate the dataTransfer</title>
<script src="../../resources/crossorigin.sub.js"></script>
<style type="text/css">
div {
  width:105px;
  min-height:105px;
  text-align:center;
  margin-top:20px;
  padding:10px;
  border:solid thin navy;
}
p:first-child {
  padding-left:12px;
}
#image { visibility: hidden; }
</style>
</head>
<body>
<p>
  <canvas width="100" height="100" draggable="true">Canvas</canvas>
</p>
<p>Drag the navy square above to the box below.</p>
<div></div>
<p><img id="image" alt="" width="100" height="100" /></p>

<script><![CDATA[
document.getElementsByTagName("img")[0].src = crossOriginUrl("www", "../../resources/100x100-navy.png");

window.onload = function() {
  var canvas = document.getElementsByTagName('canvas')[0], div = document.getElementsByTagName('div')[0], failed = [];
  var context = canvas.getContext('2d');
  var image = document.getElementById('image');
  context.drawImage(image, 0, 0);
  div.ondragover = div.ondragenter = function(e) {
    e.preventDefault();
    e.dataTransfer.dropEffect = 'copy';
  };
  div.ondrop = canvas.ondragstart = function(e) {
    if( e.type == 'dragstart' ) {
      e.dataTransfer.setData('Text', 'dummy text');
      e.dataTransfer.dropEffect = 'copy';
    }
    for( var i = 0; i < e.dataTransfer.types.length; i++ ) {
      if( e.dataTransfer.types[i].match(/image\//) ) {
        failed[failed.length] = e.dataTransfer.types[i];
      }
    }
    if( e.type == 'drop' ) {
      e.preventDefault();
      document.getElementsByTagName('p')[1].innerHTML = failed.length ? ( 'FAIL (found ' + failed.join() + ')' ) : 'PASS';
    }
  };
};
]]></script>
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
        "byte_end": 247,
        "byte_start": 224,
        "col": 1,
        "line": 6
      }
    },
    {
      "category": "Html",
      "code": "html.img.src_or_srcset.missing",
      "message": "Element “img” is missing one or more of the following attributes: “src”, “srcset”.",
      "severity": "Warning",
      "span": {
        "byte_end": 661,
        "byte_start": 611,
        "col": 4,
        "line": 27
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
  "source_name": "html/editing/dnd/canvas/cross-domain/001-manual.xhtml"
}
```
