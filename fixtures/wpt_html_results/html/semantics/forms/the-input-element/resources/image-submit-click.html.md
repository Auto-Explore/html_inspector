# html/semantics/forms/the-input-element/resources/image-submit-click.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-input-element/resources/image-submit-click.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<form>
  <input type="image" name="name" value="value">
</form>

<script>
"use strict";
if (window.location.search.startsWith("?name.x")) {
  // The action pointed to ourself, so the form submitted something
  window.parent.success(window.location.href);
} else {
  const input = document.querySelector("input");
  input.click();
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
        "byte_end": 71,
        "byte_start": 25,
        "col": 3,
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
  "source_name": "html/semantics/forms/the-input-element/resources/image-submit-click.html"
}
```
