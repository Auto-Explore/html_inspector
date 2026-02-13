# html/semantics/scripting-1/the-template-element/template-table-crash.html

Counts:
- errors: 1
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/scripting-1/the-template-element/template-table-crash.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<link rel="author" href="mailto:masonf@chromium.org">
<link rel="help" href="https://crbug.com/1212619">
<meta name="assert" content="The renderer should not crash.">

<template id=tmpl></template>
<table id=tbl>
  <script>
    tmpl.appendChild(tbl);
  </script>
  Crash
</table>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.parser.misplaced_table_text",
      "message": "Misplaced non-space characters inside a table.",
      "severity": "Error",
      "span": {
        "byte_end": 310,
        "byte_start": 301,
        "col": 12,
        "line": 11
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
  "source_name": "html/semantics/scripting-1/the-template-element/template-table-crash.html"
}
```
