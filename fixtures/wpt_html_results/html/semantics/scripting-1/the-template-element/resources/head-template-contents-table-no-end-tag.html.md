# html/semantics/scripting-1/the-template-element/resources/head-template-contents-table-no-end-tag.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/scripting-1/the-template-element/resources/head-template-contents-table-no-end-tag.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
<head>
  <title>The file contains template element with open table, tr, td tags, but without end td, tr, table tags, in the head</title>
  <link rel="author" title="Aleksei Yu. Semenov" href="mailto:a.semenov@unipro.ru">
  <template>
    <table>
      <tr>
        <td>Hello, cell one!
  </template>
</head>
<body>
</body>
</html>
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
  "source_name": "html/semantics/scripting-1/the-template-element/resources/head-template-contents-table-no-end-tag.html"
}
```
