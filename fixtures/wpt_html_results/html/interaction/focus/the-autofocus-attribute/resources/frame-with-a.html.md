# html/interaction/focus/the-autofocus-attribute/resources/frame-with-a.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/interaction/focus/the-autofocus-attribute/resources/frame-with-a.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<body>
<a name="anchor1"></a>
</body>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.a.name.obsolete",
      "message": "The “name” attribute on the “a” element is obsolete. Consider putting an “id” attribute on the nearest container instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 41,
        "byte_start": 23,
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
  "source_name": "html/interaction/focus/the-autofocus-attribute/resources/frame-with-a.html"
}
```
