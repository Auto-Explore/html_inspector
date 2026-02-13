# html/editing/dnd/interactiveelements/scrollable_inside_draggable_element.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/editing/dnd/interactiveelements/scrollable_inside_draggable_element.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<html>
  <head>
    <title>Scrollable element inside draggable element</title>
    <style type="text/css">
div div { width: 300px; height: 100px; overflow: auto; border: 1px solid orange; }
    </style>
    <script type="text/javascript">
window.onload = function () {
  document.getElementsByTagName('div')[0].ondragstart = function (e) {
    e.dataTransfer.effectAllowed = 'copy';
    e.dataTransfer.setData('Text', 'dummy text');
  };
};
    </script>
  </head>
  <body>

    <p>It should be possible to drag the scrollbar thumbs of the box below without dragging the whole box.</p>
    <div draggable="true">
      <div>Dummy text<br>Dummy text<br>Dummy text<br>Dummy text<br>Dummy text<br>Dummy text<br>Dummy text<br>Dummy text<br>Dummy text<br>Dummy text<br>Dummy text<br>xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx</div>
    </div>

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
        "byte_end": 122,
        "byte_start": 99,
        "col": 5,
        "line": 5
      }
    },
    {
      "category": "Html",
      "code": "html.script.type.unnecessary",
      "message": "The “type” attribute is unnecessary for JavaScript resources.",
      "severity": "Warning",
      "span": {
        "byte_end": 254,
        "byte_start": 223,
        "col": 5,
        "line": 8
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
  "source_name": "html/editing/dnd/interactiveelements/scrollable_inside_draggable_element.html"
}
```
