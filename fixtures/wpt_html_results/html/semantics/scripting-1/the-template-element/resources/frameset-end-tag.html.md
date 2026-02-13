# html/semantics/scripting-1/the-template-element/resources/frameset-end-tag.html

Counts:
- errors: 2
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/scripting-1/the-template-element/resources/frameset-end-tag.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
<head>
    <title>The file contains frameset with the template and frameset end tag in it</title>
    <link rel="author" title="Sergey G. Grekhov" href="mailto:sgrekhov@unipro.ru">
</head>
<frameset>
    <template></frameset></template>
</frameset>
</html>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.element.frameset.obsolete",
      "message": "The “frameset” element is obsolete. Use the “iframe” element and CSS instead, or use server-side includes.",
      "severity": "Warning",
      "span": {
        "byte_end": 222,
        "byte_start": 212,
        "col": 1,
        "line": 7
      }
    },
    {
      "category": "Html",
      "code": "html.parser.stray_end_tag",
      "message": "Stray end tag “template”.",
      "severity": "Error",
      "span": {
        "byte_end": 259,
        "byte_start": 248,
        "col": 26,
        "line": 8
      }
    },
    {
      "category": "Html",
      "code": "html.parser.stray_end_tag",
      "message": "Stray end tag “frameset”.",
      "severity": "Error",
      "span": {
        "byte_end": 271,
        "byte_start": 260,
        "col": 1,
        "line": 9
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
  "source_name": "html/semantics/scripting-1/the-template-element/resources/frameset-end-tag.html"
}
```
