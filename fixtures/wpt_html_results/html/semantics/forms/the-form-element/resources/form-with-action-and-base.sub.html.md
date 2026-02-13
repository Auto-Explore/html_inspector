# html/semantics/forms/the-form-element/resources/form-with-action-and-base.sub.html

Counts:
- errors: 1
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-form-element/resources/form-with-action-and-base.sub.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<base href="target/"></base>

<form action="{{GET[action]}}">
  <input type="text" name="name" value="value">
  <input type="submit" value="Submit">
</form>

<script>
"use strict";

if (window.location.search.startsWith("?name=value")) {
  // The action pointed to ourself, so the form submitted something
  window.parent.success(window.location.href);
} else {
  const form = document.querySelector("form");
  form.submit();
}
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.parser.stray_end_tag",
      "message": "Stray end tag “base”.",
      "severity": "Error",
      "span": {
        "byte_end": 44,
        "byte_start": 37,
        "col": 22,
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
  "source_name": "html/semantics/forms/the-form-element/resources/form-with-action-and-base.sub.html"
}
```
