# html/browsers/origin/relaxing-the-same-origin-restriction/support/document_domain_setter_iframe.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/origin/relaxing-the-same-origin-restriction/support/document_domain_setter_iframe.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<html>
  <head>
    <title></title>
    <script src="/common/get-host-info.sub.js"></script>
    <script>
      document.domain = get_host_info().ORIGINAL_HOST;
    </script>
  </head>
  <body>
  </body>
</html>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.title.empty",
      "message": "Element “title” must not be empty.",
      "severity": "Warning",
      "span": {
        "byte_end": 43,
        "byte_start": 36,
        "col": 5,
        "line": 4
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
  "source_name": "html/browsers/origin/relaxing-the-same-origin-restriction/support/document_domain_setter_iframe.html"
}
```
