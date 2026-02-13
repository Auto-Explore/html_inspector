# html/rendering/replaced-elements/embedded-content-rendering-rules/canvas-update-with-border-object-fit-ref.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/replaced-elements/embedded-content-rendering-rules/canvas-update-with-border-object-fit-ref.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<div style="width: 200px; height: 100px; background: black; border: 100px solid blue; padding-left: 100px">
  <div style="width: 100px; height: 100px; background: green"></div>
</div>
```

```json
{
  "messages": [
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
  "source_name": "html/rendering/replaced-elements/embedded-content-rendering-rules/canvas-update-with-border-object-fit-ref.html"
}
```
