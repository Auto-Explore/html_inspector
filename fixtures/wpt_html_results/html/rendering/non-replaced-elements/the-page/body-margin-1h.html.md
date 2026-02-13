# html/rendering/non-replaced-elements/the-page/body-margin-1h.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/the-page/body-margin-1h.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset=utf-8>
<title>Test that body's style margin takes precedence over everything else in quirks mode</title>
<link rel=match href="body-margin-1-ref.html">
<iframe marginwidth="20" src="data:text/html,<body marginwidth='30' leftmargin='40' rightmargin='50' style='margin-left: 100px; margin-right: 100px'>100px left/right margins, default top/bottom margins</body>"></iframe>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.iframe.src.invalid",
      "message": "Bad value “data:text/html,<body marginwidth='30' leftmargin='40' rightmargin='50' style='margin-left: 100px; margin-right: 100px'>100px left/right margins, default top/bottom margins</body>” for attribute “src” on element “iframe”.",
      "severity": "Warning",
      "span": {
        "byte_end": 392,
        "byte_start": 182,
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
  "source_name": "html/rendering/non-replaced-elements/the-page/body-margin-1h.html"
}
```
