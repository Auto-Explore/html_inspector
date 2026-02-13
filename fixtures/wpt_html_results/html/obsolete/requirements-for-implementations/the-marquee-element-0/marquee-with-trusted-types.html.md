# html/obsolete/requirements-for-implementations/the-marquee-element-0/marquee-with-trusted-types.html

Counts:
- errors: 0
- warnings: 4
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/obsolete/requirements-for-implementations/the-marquee-element-0/marquee-with-trusted-types.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<link rel="help" href="github.com/whatwg/html/issues/10249">
<meta http-equiv="Content-Security-Policy" content="require-trusted-types-for 'script'">
<link rel="match" href="marquee-with-trusted-types-ref.html">
<link rel="match" href="marquee-with-trusted-types-alternate-ref.html">

<marquee scrollamount="0">
  <div style="width: 10px; height: 10px; background-color: green;"></div>
</marquee>

```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.meta.csp.invalid",
      "message": "Bad value “require-trusted-types-for 'script'” for attribute “content” on element “meta”.",
      "severity": "Warning",
      "span": {
        "byte_end": 165,
        "byte_start": 77,
        "col": 1,
        "line": 3
      }
    },
    {
      "category": "Html",
      "code": "html.element.marquee.obsolete",
      "message": "The “marquee” element is obsolete. Use CSS instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 327,
        "byte_start": 301,
        "col": 1,
        "line": 7
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
  "source_name": "html/obsolete/requirements-for-implementations/the-marquee-element-0/marquee-with-trusted-types.html"
}
```
