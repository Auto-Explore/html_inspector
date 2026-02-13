# html/rendering/the-details-element/details-crash.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/the-details-element/details-crash.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE HTML>
<link rel="help" href="https://html.spec.whatwg.org/multipage/rendering.html#the-details-and-summary-elements">
<link rel="help" href="https://drafts.csswg.org/css-pseudo-4/#details-content-pseudo">
<link rel="help" href="https://bugzilla.mozilla.org/show_bug.cgi?id=1959597">

<!-- This test passes if it does not crash. -->

<style>
  :is([open],) {}
</style>
<details open></details>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.details.missing_summary",
      "message": "Element “details” is missing a required instance of child element “summary”.",
      "severity": "Warning",
      "span": {
        "byte_end": 402,
        "byte_start": 392,
        "col": 15,
        "line": 11
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
  "source_name": "html/rendering/the-details-element/details-crash.html"
}
```
