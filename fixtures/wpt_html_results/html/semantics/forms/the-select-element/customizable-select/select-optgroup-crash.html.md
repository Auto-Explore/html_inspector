# html/semantics/forms/the-select-element/customizable-select/select-optgroup-crash.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-select-element/customizable-select/select-optgroup-crash.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
<link rel=author href="mailto:jarhar@chromium.org">
<link rel=help href="https://issues.chromium.org/issues/398077423">
<script>
const div = document.createElement('div');
const optgroup = document.createElement('optgroup');
const option = document.createElement('option');
const select = document.createElement('select');
div.appendChild(option);
div.appendChild(optgroup);
select.appendChild(div);
</script>
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
  "source_name": "html/semantics/forms/the-select-element/customizable-select/select-optgroup-crash.html"
}
```
