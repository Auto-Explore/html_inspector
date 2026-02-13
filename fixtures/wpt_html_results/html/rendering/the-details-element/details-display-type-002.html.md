# html/rendering/the-details-element/details-display-type-002.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/the-details-element/details-display-type-002.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE HTML>
<meta charset="utf-8">
<title>Details display property (flex)</title>
<link rel="match" href="details-display-type-002-ref.html">
<link rel="help" href="https://github.com/whatwg/html/pull/10265">
<link rel="help" href="https://bugzilla.mozilla.org/show_bug.cgi?id=1856374">
<link rel="help" href="https://github.com/dbaron/details-styling">
<style>
  details {
    display: flex;
    flex-direction: column;
    gap: 3em;
  }
</style>
<details open>
  <summary>The summary.</summary>
  Some details.
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
  "source_name": "html/rendering/the-details-element/details-display-type-002.html"
}
```
