# html/semantics/embedded-content/the-img-element/document-base-url.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-img-element/document-base-url.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE HTML>
<meta charset="utf-8">
<title>Document base URL img test</title>
<link rel="help" href="https://html.spec.whatwg.org/multipage/#the-img-element" />
<link rel="match" href="document-base-url-ref.html">
<base href="resources/" />
<img src="cat.jpg" alt="cat">
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
        "byte_end": 243,
        "byte_start": 217,
        "col": 1,
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
  "source_name": "html/semantics/embedded-content/the-img-element/document-base-url.html"
}
```
