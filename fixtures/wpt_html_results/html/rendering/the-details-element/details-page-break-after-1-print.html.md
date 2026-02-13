# html/rendering/the-details-element/details-page-break-after-1-print.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/the-details-element/details-page-break-after-1-print.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<!-- Any copyright is dedicated to the Public Domain.
   - http://creativecommons.org/publicdomain/zero/1.0/ -->

<html>
  <link rel="match" href="details-two-pages-print-ref.html">
  <link rel="help" href="https://html.spec.whatwg.org/multipage/rendering.html#the-details-and-summary-elements">
  <style>
  summary {
    /* Hide the triangle for comparing with div in reftest. */
    list-style-type: none;
  }
  </style>
  <body>
    <details open>
      <summary style="page-break-after: always;">Summary</summary>
      <p>This is the details.</p>
    </details>
  </body>
</html>
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
  "source_name": "html/rendering/the-details-element/details-page-break-after-1-print.html"
}
```
