# html/rendering/non-replaced-elements/form-controls/select-fixedpos-crash.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/form-controls/select-fixedpos-crash.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset=utf-8>
<link rel=author href="mailto:emilio@crisal.io" title="Emilio Cobos Álvarez">
<link rel=author href="https://mozilla.org" title="Mozilla">
<link rel=help href="https://bugzilla.mozilla.org/show_bug.cgi?id=1741776">
<style>
#a {
  rotate: 1deg 1 0 44;
  filter: drop-shadow(81px 6px 0px red);
}
</style>
<script>
window.onload = function() {
  document.getElementById("b").appendChild(document.getElementById("c"));
}
</script>
<select id="a" multiple="multiple">
  <option id="b">x</option>
</select>
<div id="c" style="position: fixed; top: 0; left: 0;">
  x
</div>
```

```json
{
  "messages": [
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
  "source_name": "html/rendering/non-replaced-elements/form-controls/select-fixedpos-crash.html"
}
```
