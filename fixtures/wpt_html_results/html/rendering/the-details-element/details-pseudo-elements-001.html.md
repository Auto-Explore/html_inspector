# html/rendering/the-details-element/details-pseudo-elements-001.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/the-details-element/details-pseudo-elements-001.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE HTML>
<title>Details pseudo-elements</title>
<link rel="match" href="details-pseudo-elements-001-ref.html">
<link rel="help" href="https://github.com/whatwg/html/pull/10265">
<link rel="help" href="https://drafts.csswg.org/css-pseudo-4/#details-content-pseudo">
<link rel="help" href="https://github.com/dbaron/details-styling">
<link rel="help" href="https://bugs.chromium.org/p/chromium/issues/detail?id=1469418">
<style>

details::details-content {
  background: aqua;
  display: block; /* override display: contents for slot */
}

</style>

<details open>
  <summary>The summary</summary>
  <div>The details!</div>
</details>
```

```json
{
  "messages": [
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/rendering/the-details-element/details-pseudo-elements-001.html"
}
```
