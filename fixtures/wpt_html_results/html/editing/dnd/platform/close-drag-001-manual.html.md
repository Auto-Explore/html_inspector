# html/editing/dnd/platform/close-drag-001-manual.html

Counts:
- errors: 0
- warnings: 4
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/editing/dnd/platform/close-drag-001-manual.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>drag &amp; drop - closing a popup while a drag is in operation</title>
<style type="text/css">
p.gone, ul { display: none; }
p.gone + ul { display: block; }
</style>
<script type="text/javascript">
window.onload = function() {
  if( location.href.match(/#popup$/) ) { document.getElementsByTagName('p')[0].className = 'gone'; }
  document.getElementsByTagName('ul')[0].ondragstart = function(e) {
    e.dataTransfer.effectAllowed = 'copy';
    e.dataTransfer.setData('Text', 'dummy text');
    setTimeout(function () { window.close(); },100);
  };
};
</script>
<p><a href="javascript:alert('Click the link normally');" onclick="window.open('close-drag-001.html#popup','_blank');return false;">Open this page in a popup</a>.</p>
<ul draggable='true'>
  <li>Drag this text downwards, and do not release the drag.</li>
  <li>The browser may optionally cancel the drag. The browser may optionally close the popup. Fail if the drag placeholder gets stuck. Fail if the browser crashes. Fail if anything horrible happens. Fail if your pet kitten gets sick.</li>
  <li>Release the drag.</li>
  <li>Fail if the drag placeholder gets stuck. Fail if the browser crashes.</li>
</ul>
<noscript><p>Enable JavaScript and reload</p></noscript>
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
        "byte_end": 117,
        "byte_start": 94,
        "col": 1,
        "line": 3
      }
    },
    {
      "category": "Html",
      "code": "html.script.type.unnecessary",
      "message": "The “type” attribute is unnecessary for JavaScript resources.",
      "severity": "Warning",
      "span": {
        "byte_end": 220,
        "byte_start": 189,
        "col": 1,
        "line": 7
      }
    },
    {
      "category": "Html",
      "code": "html.a.href.invalid",
      "message": "Bad value “javascript:alert('Click the link normally');” for attribute “href” on element “a”.",
      "severity": "Warning",
      "span": {
        "byte_end": 716,
        "byte_start": 587,
        "col": 4,
        "line": 17
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
  "source_name": "html/editing/dnd/platform/close-drag-001-manual.html"
}
```
