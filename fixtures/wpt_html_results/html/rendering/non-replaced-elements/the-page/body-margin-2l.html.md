# html/rendering/non-replaced-elements/the-page/body-margin-2l.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/the-page/body-margin-2l.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset=utf-8>
<title>Test that body's topmargin/bottommargin attributes take precedence over iframe marginheight in quirks mode</title>
<link rel=match href="body-margin-2-ref.html">
<iframe marginheight="20" src="data:text/html,<body topmargin='100' bottommargin='100'>100px top/bottom margins, default left/right margins</body>"></iframe>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.iframe.src.invalid",
      "message": "Bad value “data:text/html,<body topmargin='100' bottommargin='100'>100px top/bottom margins, default left/right margins</body>” for attribute “src” on element “iframe”.",
      "severity": "Warning",
      "span": {
        "byte_end": 354,
        "byte_start": 206,
        "col": 1,
        "line": 5
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
  "source_name": "html/rendering/non-replaced-elements/the-page/body-margin-2l.html"
}
```
