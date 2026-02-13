# html/the-xhtml-syntax/parsing-xhtml-documents/data-xhtml-with-dtd-ref.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/the-xhtml-syntax/parsing-xhtml-documents/data-xhtml-with-dtd-ref.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
Test passes if it correctly shows &Aacute; in the subframe.
<hr>
<iframe srcdoc="&amp;Aacute"></iframe>
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
  "source_name": "html/the-xhtml-syntax/parsing-xhtml-documents/data-xhtml-with-dtd-ref.html"
}
```
