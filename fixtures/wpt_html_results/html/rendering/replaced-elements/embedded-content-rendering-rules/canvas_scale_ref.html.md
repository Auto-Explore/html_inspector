# html/rendering/replaced-elements/embedded-content-rendering-rules/canvas_scale_ref.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/replaced-elements/embedded-content-rendering-rules/canvas_scale_ref.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset="utf-8">
<style>
span {
  display: inline-block;
  width: 20px;
  height: 20px;
}
div {
  line-height: 0;
}
</style>
<div><span style="background-color: #FF00FF"></span><span style="background-color: #00FF00"></span></div>
<div><span style="background-color: #0000FF"></span><span style="background-color: #FF00FF"></span></div>
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
  "source_name": "html/rendering/replaced-elements/embedded-content-rendering-rules/canvas_scale_ref.html"
}
```
