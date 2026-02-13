# html/semantics/embedded-content/the-iframe-element/srcdoc-removed-iframe-crash.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-iframe-element/srcdoc-removed-iframe-crash.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
    <body>
        <title>iframe with srcdoc content that loads after iframe is removed from the document</title>
        <link rel="author" title="Martin Robinson" href="mrobinson@igalia.com">
        <link rel="help" href="https://github.com/servo/servo/issues/32432">
        <iframe srcdoc="contents"></iframe>
        <script>
            document.querySelector('iframe').remove();
        </script>
    </body>
</html>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.link.in_body.disallowed_rel",
      "message": "A “link” element must not appear as a descendant of a “body” element unless the “link” element has an “itemprop” attribute or has a “rel” attribute whose value contains “dns-prefetch”, “modulepreload”, “pingback”, “preconnect”, “prefetch”, “preload”, “prerender”, or “stylesheet”.",
      "severity": "Warning",
      "span": {
        "byte_end": 216,
        "byte_start": 145,
        "col": 9,
        "line": 5
      }
    },
    {
      "category": "Html",
      "code": "html.link.in_body.disallowed_rel",
      "message": "A “link” element must not appear as a descendant of a “body” element unless the “link” element has an “itemprop” attribute or has a “rel” attribute whose value contains “dns-prefetch”, “modulepreload”, “pingback”, “preconnect”, “prefetch”, “preload”, “prerender”, or “stylesheet”.",
      "severity": "Warning",
      "span": {
        "byte_end": 293,
        "byte_start": 225,
        "col": 9,
        "line": 6
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
  "source_name": "html/semantics/embedded-content/the-iframe-element/srcdoc-removed-iframe-crash.html"
}
```
