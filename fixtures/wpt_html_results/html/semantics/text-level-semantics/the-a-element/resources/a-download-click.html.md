# html/semantics/text-level-semantics/the-a-element/resources/a-download-click.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/text-level-semantics/the-a-element/resources/a-download-click.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<a id="blob-url" download="foo.html">Click me</a>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.a.href.required_for_download",
      "message": "Element “a” is missing required attribute “href”.",
      "severity": "Warning",
      "span": {
        "byte_end": 53,
        "byte_start": 16,
        "col": 1,
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
  "source_name": "html/semantics/text-level-semantics/the-a-element/resources/a-download-click.html"
}
```
