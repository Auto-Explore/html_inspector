# html/semantics/embedded-content/the-img-element/content-type-with-parameters-ref.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-img-element/content-type-with-parameters-ref.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
<head>
    <title>SVG is recognized even when the 'Content-Type' header includes parameters</title>
    <style>
        #box {
           width: 100px;
           height: 100px;
           background-color: red;
        }
    </style>
</head>
<body>
   <div id="box"></div>
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
  "source_name": "html/semantics/embedded-content/the-img-element/content-type-with-parameters-ref.html"
}
```
