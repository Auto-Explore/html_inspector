# html/rendering/non-replaced-elements/tables/table-vspace-hspace-s.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/tables/table-vspace-hspace-s.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset=utf-8>
<title>table vspace hspace (standards mode)</title>
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>
<div>x</div>
<table vspace=25 hspace=25><tr><td>x</table>
<div>x</div>
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
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/rendering/non-replaced-elements/tables/table-vspace-hspace-s.html"
}
```
