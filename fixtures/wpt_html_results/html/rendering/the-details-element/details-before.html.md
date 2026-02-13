# html/rendering/the-details-element/details-before.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/the-details-element/details-before.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<link rel="author" title="Emilio Cobos Álvarez" href="mailto:emilio@crisal.io">
<link rel="help" title="https://github.com/whatwg/html/pull/3686">
<link rel="help" title="https://html.spec.whatwg.org/multipage/#the-details-element">
<link rel="mismatch" href="single-summary.html">
<title>CSS Test: details ::before pseudo-element</title>
<style>
  details::before { content: "::before" }
</style>
<details>
  <summary>This is the main summary</summary>
</details>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.link.href.required",
      "message": "A “link” element must have an “href” or “imagesrcset” attribute, or both.",
      "severity": "Warning",
      "span": {
        "byte_end": 163,
        "byte_start": 97,
        "col": 1,
        "line": 3
      }
    },
    {
      "category": "Html",
      "code": "html.link.href.required",
      "message": "A “link” element must have an “href” or “imagesrcset” attribute, or both.",
      "severity": "Warning",
      "span": {
        "byte_end": 249,
        "byte_start": 164,
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
  "source_name": "html/rendering/the-details-element/details-before.html"
}
```
