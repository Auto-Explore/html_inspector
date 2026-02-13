# html/rendering/non-replaced-elements/tables/table-vspace-hspace.html

Counts:
- errors: 1
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/tables/table-vspace-hspace.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!-- quirks -->
<meta charset=utf-8>
<title>table vspace hspace (quirks mode)</title>
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>
<div>x</div> <!-- prevent margin collapsing quirks -->
<table vspace=25 hspace=25><tr><td>x</table>
<div>x</div> <!-- prevent margin collapsing quirks -->
<script>
test(function() {
  var style = getComputedStyle(document.querySelector('table'));
  ['marginTop', 'marginRight', 'marginBottom', 'marginLeft'].forEach(function(m) {
    assert_equals(style[m], '0px', m);
  });
});
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
        "byte_end": 36,
        "byte_start": 16,
        "col": 1,
        "line": 2
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
  "source_name": "html/rendering/non-replaced-elements/tables/table-vspace-hspace.html"
}
```
