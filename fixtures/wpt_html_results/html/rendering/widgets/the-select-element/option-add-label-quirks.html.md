# html/rendering/widgets/the-select-element/option-add-label-quirks.html

Counts:
- errors: 1
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/widgets/the-select-element/option-add-label-quirks.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<meta charset="utf-8">
<title>OPTION's label attribute in SELECT -- Adding a label (quirks)</title>
<link rel="help" href="https://html.spec.whatwg.org/multipage/rendering.html#the-select-element-2">
<link rel="match" href="option-label-ref.html">
<meta name="assert" content="An option element is expected to be rendered by displaying the element's label when the document is in quirks mode">

<select>
  <option>Element Text</option>
</select>
<br/>
<select size="4">
  <option>Element Text</option>
</select>
<script>
let options = document.querySelectorAll("option");
options[0].getBoundingClientRect(); // force layout.
for (let option of options) {
  option.setAttribute("label", "Label Text");
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
        "byte_end": 22,
        "byte_start": 0,
        "col": 1,
        "line": 1
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
  "source_name": "html/rendering/widgets/the-select-element/option-add-label-quirks.html"
}
```
