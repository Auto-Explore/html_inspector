# html/semantics/interactive-elements/the-details-element/auto-expand-ax-slot-recalc-crash.html

Counts:
- errors: 1
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/interactive-elements/the-details-element/auto-expand-ax-slot-recalc-crash.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<link rel=help href="https://bugs.chromium.org/p/chromium/issues/detail?id=1255406">
<script type="text/javascript">
var nodes = Array();
var text = Array();
 {
 nodes[9] = document.createElement('textarea');
 nodes[11] = document.createElement('legend');
 nodes[44] = document.createElement('details');
 document.documentElement.appendChild(nodes[44]);
 nodes[68] = document.createElement('fieldset');
 nodes[81] = document.createElement('option');



 nodes[85] = document.createElement('img');
 text[42] = document.createTextNode('744879385');
 nodes[44].appendChild(text[42]);
 nodes[68].appendChild(nodes[11]);
 nodes[44].appendChild(nodes[68]);
 requestAnimationFrame(() => {
  requestAnimationFrame(() => {
   try { nodes[85].appendChild(nodes[68]); } catch(e) {}
  });
 });
 nodes[44].appendChild(nodes[9]);
 requestAnimationFrame(() => { document.execCommand("SelectAll", false, ""); });
}
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.parser.start_tag_without_doctype",
      "message": "Start tag seen without seeing a doctype first. Expected “<!DOCTYPE html>”.",
      "severity": "Error",
      "span": {
        "byte_end": 84,
        "byte_start": 0,
        "col": 1,
        "line": 1
      }
    },
    {
      "category": "Html",
      "code": "html.script.type.unnecessary",
      "message": "The “type” attribute is unnecessary for JavaScript resources.",
      "severity": "Warning",
      "span": {
        "byte_end": 116,
        "byte_start": 85,
        "col": 1,
        "line": 2
      }
    },
    {
      "category": "Html",
      "code": "html.head.title.missing",
      "message": "Element “head” is missing a required instance of child element “title”.",
      "severity": "Warning",
      "span": null
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/semantics/interactive-elements/the-details-element/auto-expand-ax-slot-recalc-crash.html"
}
```
