# html/semantics/document-metadata/the-base-element/example.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/document-metadata/the-base-element/example.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>Example</title>
<base target="targetWin" href="">
<base target="_self" href="http://www.example.com/">
<link rel="author" title="Intel" href="http://www.intel.com/">
<a id="a1" href="example2.html" target="">click me</a>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.target.empty",
      "message": "Bad value “” for attribute “target” on element “a”.",
      "severity": "Warning",
      "span": {
        "byte_end": 254,
        "byte_start": 212,
        "col": 1,
        "line": 7
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
  "source_name": "html/semantics/document-metadata/the-base-element/example.html"
}
```
