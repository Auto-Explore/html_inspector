# html/editing/dnd/platform/moving-window.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/editing/dnd/platform/moving-window.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>drag &amp; drop - moving windows must not start a drag</title>
<script type="text/javascript">

window.onload = function() {
  var li1 = document.getElementsByTagName('li')[3], li2 = document.getElementsByTagName('li')[4];
  li1.ondragstart = li2.ondragstart = function(e) {
    e.dataTransfer.effectAllowed = 'copy';
    e.dataTransfer.setData('Text', 'dummy text');
    this.innerHTML = 'FAIL';
  };
  li1.onmousedown = function () { window.moveBy(0,10); };
  li2.onmousedown = function () { setTimeout(function () {
    window.moveBy(0,10);
  },10); };
};

</script>
<p></p>
<ol>
  <li onclick="window.open(location.href,'_blank','width=500,height=300');">Click here to open this page in a popup window.</li>
  <li>Ensure that this popup window is not maximised (or tab, in the case of tabs being rendered as an MDI).</li>
  <li>Ensure that your browser settings allow browser windows to be moved by scripts.</li>
  <li draggable='true'>Press your mouse down on this text but do not move it afterwards. Fail if a drag operation has started (eg. if the mouse cursor shows that you are dragging something, or if some drag placeholder text appears, or if this text changes).</li>
  <li draggable='true'>Press your mouse down on this text but do not move it afterwards. Fail if a drag operation has started (eg. if the mouse cursor shows that you are dragging something, or if some drag placeholder text appears, or if this text changes).</li>
</ol>
<noscript><p>Enable JavaScript and reload</p></noscript>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.script.type.unnecessary",
      "message": "The “type” attribute is unnecessary for JavaScript resources.",
      "severity": "Warning",
      "span": {
        "byte_end": 117,
        "byte_start": 86,
        "col": 1,
        "line": 3
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
  "source_name": "html/editing/dnd/platform/moving-window.html"
}
```
