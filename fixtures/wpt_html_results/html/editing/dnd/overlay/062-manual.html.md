# html/editing/dnd/overlay/062-manual.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/editing/dnd/overlay/062-manual.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<html>
  <head>
    <title>Selection spanning hidden elements</title>
    <script type="text/javascript">
window.onload = function () {
  var range = document.createRange(), p = document.getElementsByTagName('p')[0];
  range.selectNodeContents(p);
  range.setStart(p.firstChild.firstChild,4);
  range.setEnd(p.lastChild.firstChild,5);
  window.getSelection().addRange(range);
};
    </script>
  </head>
  <body>

    <p><span style="display:none">FAILPASS_</span>drag<span style="display:none">_THIS_</span>text<span style="display:none">_PASSFAIL</span></p>
    <p><textarea rows="3" cols="50"></textarea></p>
    <p>Drag the selected text into the input box. The drag placeholder should match the visible text that is being dragged. When dropped, either &quot;dragtext&quot; or &quot;PASS_drag_THIS_text_PASS&quot; should appear in the input.</p>

  </body>
</html>
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
        "byte_end": 121,
        "byte_start": 90,
        "col": 5,
        "line": 5
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
  "source_name": "html/editing/dnd/overlay/062-manual.html"
}
```
