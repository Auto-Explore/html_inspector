# html/rendering/widgets/input-checkbox-appearance-none-dynamic-crash.html

Counts:
- errors: 1
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/widgets/input-checkbox-appearance-none-dynamic-crash.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<style>
input:checked {
  appearance: none;
}
input:required {
  border-bottom-style: groove;
}
</style>
<script>
document.addEventListener("DOMContentLoaded", () => {
  a.getBoundingClientRect();
  a.defaultChecked = true;
  a.getBoundingClientRect();
})
</script>
<input id="a" required="required" type="radio">
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
        "byte_end": 7,
        "byte_start": 0,
        "col": 1,
        "line": 1
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
  "source_name": "html/rendering/widgets/input-checkbox-appearance-none-dynamic-crash.html"
}
```
