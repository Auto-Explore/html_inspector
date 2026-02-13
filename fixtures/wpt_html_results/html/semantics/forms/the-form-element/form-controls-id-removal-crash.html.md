# html/semantics/forms/the-form-element/form-controls-id-removal-crash.html

Counts:
- errors: 1
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-form-element/form-controls-id-removal-crash.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<script>
document.addEventListener("DOMContentLoaded" , () => {
  c.beginElement()
  document.onselectstart = fuzz_01
  document.execCommand("selectAll", false, null)
})
function fuzz_01() {
  document.execCommand("underline", false, null)
  c.onend = fuzz_01
  document.designMode = "on"
}
function fuzz_02() {
  d.insertAdjacentElement("beforebegin", b)
  a.appendChild(c)
  h.clientTop
  b.insertAdjacentHTML("afterend", f.innerHTML)
}
</script>
<svg>
<path id="a">
<polygon id="b">
<set id="c" onbegin="fuzz_02()" max="2s">
<strong>
<style id="d">
</style>
</strong>
<time id="f">
<form id="g">
<output id="h" form="g">
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
        "byte_end": 8,
        "byte_start": 0,
        "col": 1,
        "line": 1
      }
    },
    {
      "category": "Html",
      "code": "html.style.not_allowed_here",
      "message": "Element “style” not allowed as child of “strong” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 551,
        "byte_start": 537,
        "col": 1,
        "line": 24
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
  "source_name": "html/semantics/forms/the-form-element/form-controls-id-removal-crash.html"
}
```
