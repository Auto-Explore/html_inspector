# html/rendering/non-replaced-elements/the-page/body-margin-2g.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/the-page/body-margin-2g.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset=utf-8>
<title>Test that body's style margin takes precedence over everything else in standards mode</title>
<link rel=match href="body-margin-2-ref.html">
<iframe marginheight="20" src="data:text/html,<!doctype html><body marginheight='30' topmargin='40' bottommargin='50' style='margin-top: 100px; margin-bottom: 100px'>100px top/bottom margins, default left/right margins</body>"></iframe>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.iframe.src.invalid",
      "message": "Bad value “data:text/html,<!doctype html><body marginheight='30' topmargin='40' bottommargin='50' style='margin-top: 100px; margin-bottom: 100px'>100px top/bottom margins, default left/right margins</body>” for attribute “src” on element “iframe”.",
      "severity": "Warning",
      "span": {
        "byte_end": 412,
        "byte_start": 185,
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
  "source_name": "html/rendering/non-replaced-elements/the-page/body-margin-2g.html"
}
```
