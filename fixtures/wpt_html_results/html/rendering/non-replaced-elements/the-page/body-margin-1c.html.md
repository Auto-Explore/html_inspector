# html/rendering/non-replaced-elements/the-page/body-margin-1c.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/the-page/body-margin-1c.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset=utf-8>
<title>Test that body's leftmargin/rightmargin attributes have the right effect in standards mode</title>
<link rel=match href="body-margin-1-ref.html">
<iframe src="data:text/html,<!doctype html><body leftmargin='100' rightmargin='100'>100px left/right margins, default top/bottom margins</body>"></iframe>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.iframe.src.invalid",
      "message": "Bad value “data:text/html,<!doctype html><body leftmargin='100' rightmargin='100'>100px left/right margins, default top/bottom margins</body>” for attribute “src” on element “iframe”.",
      "severity": "Warning",
      "span": {
        "byte_end": 335,
        "byte_start": 190,
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
  "source_name": "html/rendering/non-replaced-elements/the-page/body-margin-1c.html"
}
```
