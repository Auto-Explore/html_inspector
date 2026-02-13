# html/semantics/forms/constraints/number-input-lang-validationMessage-crash.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/constraints/number-input-lang-validationMessage-crash.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>Should not assert or crash</title>
<link rel="help" href="https://bugzilla.mozilla.org/show_bug.cgi?id=1649569">
<script>
window.onload = () => {
  var a = document.getElementById("a")
  var c = document.createElement("m")
  a.valueAsNumber = 0.165
  c.insertBefore(b, c.childNodes[0])
  a.validationMessage
}
</script>
<pre lang="nb">
<form id="b">
<input id="a" type="number">
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
  "source_name": "html/semantics/forms/constraints/number-input-lang-validationMessage-crash.html"
}
```
