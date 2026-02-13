# html/rendering/non-replaced-elements/the-hr-element-0/width.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/the-hr-element-0/width.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset="utf-8">
<title></title>
<link rel="match" href="width-ref.html">
<hr>
<hr width='50%'>
<hr width='100'>
<hr width='100foo'>
<hr width='  100    '>
<hr width='100.99'>
<hr width='0'>
<hr width='00'>
<hr width='+0'>
<hr width='+00'>
<hr width='++0'>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.title.empty",
      "message": "Element “title” must not be empty.",
      "severity": "Warning",
      "span": {
        "byte_end": 46,
        "byte_start": 39,
        "col": 1,
        "line": 3
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
  "source_name": "html/rendering/non-replaced-elements/the-hr-element-0/width.html"
}
```
