# html/links/icon/empty-href-crash.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/links/icon/empty-href-crash.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset="utf-8">
<title>Test for #41047: Panic when the href attribute of a link is empty.</title>
<link rel="icon" href="">
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.link.href.empty",
      "message": "Bad value “” for attribute “href” on element “link”.",
      "severity": "Warning",
      "span": {
        "byte_end": 146,
        "byte_start": 121,
        "col": 1,
        "line": 4
      }
    },
    {
      "category": "Html",
      "code": "html.url.empty",
      "message": "Bad value “” for attribute “href” on element “link”.",
      "severity": "Warning",
      "span": {
        "byte_end": 146,
        "byte_start": 121,
        "col": 1,
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
  "source_name": "html/links/icon/empty-href-crash.html"
}
```
