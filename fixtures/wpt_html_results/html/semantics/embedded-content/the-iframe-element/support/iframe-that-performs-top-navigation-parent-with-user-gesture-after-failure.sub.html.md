# html/semantics/embedded-content/the-iframe-element/support/iframe-that-performs-top-navigation-parent-with-user-gesture-after-failure.sub.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-iframe-element/support/iframe-that-performs-top-navigation-parent-with-user-gesture-after-failure.sub.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>

<iframe
  src="http://{{hosts[alt][]}}:{{ports[http][0]}}/html/semantics/embedded-content/the-iframe-element/support/iframe-that-performs-top-navigation-child-with-user-gesture-after-failure.html"
></iframe>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.iframe.src.invalid",
      "message": "Bad value “http://{{hosts[alt][]}}:{{ports[http][0]}}/html/semantics/embedded-content/the-iframe-element/support/iframe-that-performs-top-navigation-child-with-user-gesture-after-failure.html” for attribute “src” on element “iframe”.",
      "severity": "Warning",
      "span": {
        "byte_end": 215,
        "byte_start": 17,
        "col": 1,
        "line": 3
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
  "source_name": "html/semantics/embedded-content/the-iframe-element/support/iframe-that-performs-top-navigation-parent-with-user-gesture-after-failure.sub.html"
}
```
