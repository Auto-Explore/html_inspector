# html/rendering/non-replaced-elements/lists/ul-type-supported-ref.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/lists/ul-type-supported-ref.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset=utf-8>
<title>ul@type: supported types</title>
<style>
.disc {
  list-style-type: disc;
}
.circle {
  list-style-type: circle;
}
.square {
  list-style-type: square;
}
.none {
  list-style-type: none;
}
</style>
<ul class="disc"><li>first disc</li><li>second disc</li></ul>
<ul class="circle"><li>first circle</li><li>second circle</li></ul>
<ul class="square"><li>first square</li><li>second square</li></ul>
<ul class="none"><li>first none</li><li>second none</li></ul>
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
  "source_name": "html/rendering/non-replaced-elements/lists/ul-type-supported-ref.html"
}
```
