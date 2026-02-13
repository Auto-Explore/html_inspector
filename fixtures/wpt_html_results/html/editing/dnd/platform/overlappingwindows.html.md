# html/editing/dnd/platform/overlappingwindows.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/editing/dnd/platform/overlappingwindows.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Dropping on always-on-top application windows that overlay the browser</title>
<style>
  html, body, ol {
    margin: 0;
    padding: 0;
    height: 100%;
    width: 100%;
    background: blue;
    color: black;
    list-style-position: inside;
  }
  div {
    height: 100px;
    width: 100px;
    position: absolute;
    top: 50%;
    left: 50%;
    margin-top: -50px;
    margin-left: -50px;
    background: orange;
  }
</style>

<script type="text/javascript">

window.onload = function() {
  var orange = document.getElementsByTagName('div')[0];
  orange.ondragstart = function(e) {
    e.dataTransfer.effectAllowed = 'copy';
    e.dataTransfer.setData('Text', 'dummy text');
  };
  var blue = document.getElementsByTagName('ol')[0];
  blue.ondragenter = blue.ondragover = function(e) {
    e.preventDefault();
  };
  blue.ondrop = function(e) {
    e.preventDefault();
    this.innerHTML = 'FAIL';
  };
};

</script>

<ol>
  <li>Position the browser window so that the blue part of this page extends behind the system taskbar.</li>
  <li>Use your mouse to drag the orange box over a part of the taskbar that overlays the blue part of this page.</li>
  <li>If supported by the platform, the mouse cursor should <em>not</em> show the browser's custom &quot;copy&quot; cursor, and should instead show the system's expected cursor for dropping on that part of the taskbar.</li>
  <li>Release the drag. Fail if the text on this page changes.</li>
  <li>Reload and repeat this test for:<ul>
    <li>Where the blue part of this page extends under an always-on-top window (eg. the Windows Task Manager).</li>
    <li>Where the blue part of this page extends under an always-on-top notification (eg. a system tray info balloon).</li>
  </ul></li>
</ol>
<div draggable='true'></div>
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
        "byte_end": 486,
        "byte_start": 455,
        "col": 1,
        "line": 25
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
  "source_name": "html/editing/dnd/platform/overlappingwindows.html"
}
```
