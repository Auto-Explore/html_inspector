# html/editing/dnd/crashers/dialog-001.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/editing/dnd/crashers/dialog-001.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset='utf-8'>
<title>drag &amp; drop – crash when drag is interrupted by dialogs</title>
<style>
  body > div {
    height: 200px;
    width: 200px;
    background-color: orange;
    position: absolute;
    top: 8px;
    left: 8px;
  }
  body > div * {
    display: none;
  }
  body > div + div {
    background-color: navy;
    left: 250px;
  }
  p {
    margin-top: 220px;
  }
</style>

<script>
window.onload = function() {
  var doneonce = false;
  document.getElementsByTagName('div')[0].ondragstart = function(e) {
    alert( doneonce ? 'Dismiss this dialog. PASS if the browser does not crash.' : 'Dismiss this dialog. The browser should not crash. Without re-focusing the page first, try dragging the orange square a second time. If a second alert does not appear, release the drag, and then try dragging the orange square a third time.' );
    doneonce = true;
  };
};
</script>

<div draggable='true' itemscope></div><div></div>

<p>Try to drag the orange square onto the blue square.</p>
<noscript><p>Enable JavaScript and reload</p></noscript>
```

```json
{
  "messages": [
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/editing/dnd/crashers/dialog-001.html"
}
```
