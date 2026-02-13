# html/rendering/non-replaced-elements/tables/table-cell-width-ref.html

Counts:
- errors: 1
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/tables/table-cell-width-ref.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<style>
body {
  margin: 0;
}

.row {
  clear: both;
}

.row div {
  float: left;
}

.red {
  background-color: red;
}
</style>

<div class="row">
  <div class="red" style="width: 200px">a</div>
  <div style="width: 200px">a</div>
</div>

<div class="row">
  <div class="red" style="width: 200px">a</div>
  <div style="width: 200px">a</div>
</div>

<div class="row">
  <div class="red" style="width: 100px">a</div>
  <div style="width: 300px">a</div>
</div>

<div class="row">
  <div class="red" style="width: 100px">a</div>
  <div style="width: 300px">a</div>
</div>
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
  "source_name": "html/rendering/non-replaced-elements/tables/table-cell-width-ref.html"
}
```
