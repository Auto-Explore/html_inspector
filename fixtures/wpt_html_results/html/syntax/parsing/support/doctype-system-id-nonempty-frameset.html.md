# html/syntax/parsing/support/doctype-system-id-nonempty-frameset.html

Counts:
- errors: 1
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/syntax/parsing/support/doctype-system-id-nonempty-frameset.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html PUBLIC "-//W3C//DTD HTML 4.01 Frameset//" "http://www.w3.org/TR/html4/frameset.dtd">
<meta charset="utf-8">
<title>DOCTYPE with non-empty system identifier (Frameset)</title>

```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.parser.doctype.not_html5",
      "message": "Obsolete doctype. Expected “<!DOCTYPE html>”.",
      "severity": "Error",
      "span": {
        "byte_end": 2,
        "byte_start": 0,
        "col": 1,
        "line": 1
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
  "source_name": "html/syntax/parsing/support/doctype-system-id-nonempty-frameset.html"
}
```
