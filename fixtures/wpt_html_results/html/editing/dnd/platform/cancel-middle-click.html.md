# html/editing/dnd/platform/cancel-middle-click.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/editing/dnd/platform/cancel-middle-click.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
  <head>
    <title>Drag and drop with middle click</title>
    <style type="text/css">
div {
  width: 100px;
  height: 100px;
  background: orange;
  float: left;
}
div + div {
  background: blue;
}
div + div + div {
  background: fuchsia;
}
ol {
  clear: left;
}
    </style>
    <script type="text/javascript">
window.onload = function () {
  document.getElementsByTagName('div')[0].ondragstart = function (e) {
    e.dataTransfer.effectAllowed = 'copy';
    e.dataTransfer.setData('text','dummy text');
  };
};
    </script>
  </head>
  <body>
    <div draggable="true"></div>
    <div></div>
    <div></div>
    <noscript><p>Enable JavaScript and reload</p></noscript>
    <ol>
      <li>Drag the orange square over the blue square.</li>
      <li>Without releasing the drag, click the middle mouse button.</li>
      <li>If the platform's normal behaviour is to cancel a drag (eg. Windows and Unix+KDE), then the drag should be cancelled;<ul>
        <li>The drag placeholder should disappear, and the cursor should return to the normal mouse cursor.</li>
        <li>Move the mouse over the pink square and release the drag. The mouse cursor should remain the normal mouse cursor.</li>
      </ul></li>
      <li>If the platform's normal behaviour is not to cancel a drag (eg. Mac and Unix+Gnome), then the drag should not be cancelled;<ul>
        <li>The drag placeholder should not disappear, and the cursor should be the no-drop cursor.</li>
        <li>Move the mouse over the pink square and release the drag. The drag placeholder should disappear, and the cursor should return to the normal mouse cursor.</li>
      </ul></li>
      <li>Fail in either case if an inappropriate middle click function begins (eg. paste-and-go).</li>
    </ol>
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
        "byte_end": 110,
        "byte_start": 87,
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
        "byte_end": 336,
        "byte_start": 305,
        "col": 5,
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
  "source_name": "html/editing/dnd/platform/cancel-middle-click.html"
}
```
