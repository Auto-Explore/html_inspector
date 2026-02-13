# html/editing/dnd/file/004-manual.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/editing/dnd/file/004-manual.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>drag &amp; drop - cancelling the dropped file upload</title>
<style>
  body > div {
    height: 200px;
    width: 200px;
    background-color: orange;
  }
</style>

<script>
var fails = [];
window.onload = function() {
  var orange = document.getElementsByTagName('div')[0];
  orange.ondragover = orange.ondragenter = function(e) {
    e.preventDefault();
    e.dataTransfer.dropEffect = 'copy';
  };
  orange.ondrop = function(e) {
    //if the browser simulates a drop, it must do so with an empty FileList
    e.preventDefault();
    if( !e.dataTransfer.files ) {
      fails[fails.length] = 'No dataTransfer.files for '+e.type;
    }
    if( !window.FileList ) {
      fails[fails.length] = 'No FileList interface object';
      finish();
      return;
    }
    if( !( e.dataTransfer.files instanceof FileList ) ) {
      fails[fails.length] = 'dataTransfer.files is not a FileList';
    }
    if( e.dataTransfer.files.length ) {
      fails[fails.length] = 'dataTransfer.files.length is '+e.dataTransfer.files.length+' instead of 0 for '+e.type;
    }
    if( e.dataTransfer.files[0] ) {
      fails[fails.length] = 'dataTransfer.files[0] exists for drop';
      finish();
    }
  };

};
function finish() {
  document.getElementsByTagName('p')[0].innerHTML = fails.length ? ( 'FAIL: ' + fails.join('<br>') ) : 'PASS';
}
</script>

<div></div>

<p>Save <a href="../resources/pass.png">this image</a> to your desktop. Use your pointing device to drag the saved file from your desktop onto the orange box, and release it. <strong>A confirmation dialog must appear, allowing you to choose to cancel the upload</strong>. Refuse it. Pass if nothing happens, or if the browser simply displays the image.</p>
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
  "source_name": "html/editing/dnd/file/004-manual.html"
}
```
