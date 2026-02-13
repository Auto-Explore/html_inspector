# html/rendering/non-replaced-elements/the-page/body-margin-2-ref.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/the-page/body-margin-2-ref.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<iframe src="data:text/html,<!doctype html><body style='margin-top: 100px; margin-bottom: 100px;'>100px top/bottom margins, default left/right margins</body>"></iframe>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.iframe.src.invalid",
      "message": "Bad value “data:text/html,<!doctype html><body style='margin-top: 100px; margin-bottom: 100px;'>100px top/bottom margins, default left/right margins</body>” for attribute “src” on element “iframe”.",
      "severity": "Warning",
      "span": {
        "byte_end": 175,
        "byte_start": 16,
        "col": 1,
        "line": 2
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
  "source_name": "html/rendering/non-replaced-elements/the-page/body-margin-2-ref.html"
}
```
