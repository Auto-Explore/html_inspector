# html/semantics/forms/the-input-element/input-form-detach-style-crash.html

Counts:
- errors: 1
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-input-element/input-form-detach-style-crash.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<link rel="help" href="https://bugzilla.mozilla.org/show_bug.cgi?id=1800543">
<script>
document.addEventListener('DOMContentLoaded', () => {
  b.setCustomValidity("x")
  a.reportValidity()
  c.appendChild(a)
  document.adoptNode(d)
  document.documentElement.style.display = 'none'
})
</script>
<form id="a">
<textarea id="b">a</textarea>
</form>
<dl id="d">
<canvas id="c"></canvas>
</dl>
<input form="a">
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
        "byte_end": 77,
        "byte_start": 0,
        "col": 1,
        "line": 1
      }
    },
    {
      "category": "Html",
      "code": "html.dl.child.invalid",
      "message": "Element “canvas” not allowed as child of “dl” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 374,
        "byte_start": 359,
        "col": 1,
        "line": 15
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
  "source_name": "html/semantics/forms/the-input-element/input-form-detach-style-crash.html"
}
```
