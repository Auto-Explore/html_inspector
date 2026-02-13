# html/rendering/replaced-elements/attributes-for-embedded-content-and-images/input-type-change-from-image-1.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/replaced-elements/attributes-for-embedded-content-and-images/input-type-change-from-image-1.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel=match href="input-type-change-from-image-1-ref.html">
<input type="image" align="right">
<script>
  onload = function() {
    var i = document.querySelector("input");
    window.rect = i.getBoundingClientRect();
    i.type = "text";
  }
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.input.image.alt.missing",
      "message": "Element “input” is missing required attribute “alt”.",
      "severity": "Warning",
      "span": {
        "byte_end": 114,
        "byte_start": 80,
        "col": 1,
        "line": 3
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
  "source_name": "html/rendering/replaced-elements/attributes-for-embedded-content-and-images/input-type-change-from-image-1.html"
}
```
