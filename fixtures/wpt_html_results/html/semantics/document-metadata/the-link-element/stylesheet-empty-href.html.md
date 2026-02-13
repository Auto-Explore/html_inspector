# html/semantics/document-metadata/the-link-element/stylesheet-empty-href.html

Counts:
- errors: 0
- warnings: 4
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/document-metadata/the-link-element/stylesheet-empty-href.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset=utf-8>
<title>Test</title>
<link rel=match href=stylesheet-empty-href-ref.html>
<style>
body {
  color: green;
}
</style>
<base href=resources/empty-href.css>
<link rel=stylesheet href>
<p>This text should be green.
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.base.must_come_before_link_or_script",
      "message": "The “base” element must come before any “link” or “script” elements in the document.",
      "severity": "Warning",
      "span": {
        "byte_end": 188,
        "byte_start": 152,
        "col": 1,
        "line": 10
      }
    },
    {
      "category": "Html",
      "code": "html.link.href.empty",
      "message": "Bad value “” for attribute “href” on element “link”.",
      "severity": "Warning",
      "span": {
        "byte_end": 215,
        "byte_start": 189,
        "col": 1,
        "line": 11
      }
    },
    {
      "category": "Html",
      "code": "html.url.empty",
      "message": "Bad value “” for attribute “href” on element “link”.",
      "severity": "Warning",
      "span": {
        "byte_end": 215,
        "byte_start": 189,
        "col": 1,
        "line": 11
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
  "source_name": "html/semantics/document-metadata/the-link-element/stylesheet-empty-href.html"
}
```
