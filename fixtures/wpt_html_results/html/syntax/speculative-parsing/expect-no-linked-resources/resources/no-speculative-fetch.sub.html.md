# html/syntax/speculative-parsing/expect-no-linked-resources/resources/no-speculative-fetch.sub.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/syntax/speculative-parsing/expect-no-linked-resources/resources/no-speculative-fetch.sub.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>Navigating to a page with expect-no-linked-resources</title>

<body>
  <script>
    document.write('<plaintext>');
  </script>
  <img
    src="/html/syntax/speculative-parsing/expect-no-linked-resources/resources/stash.py?action=put&uuid={{GET[uuid]}}">
</body>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 276,
        "byte_start": 152,
        "col": 3,
        "line": 8
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
  "source_name": "html/syntax/speculative-parsing/expect-no-linked-resources/resources/no-speculative-fetch.sub.html"
}
```
