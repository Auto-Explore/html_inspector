# html/semantics/embedded-content/the-object-element/object-image-only-for-print.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-object-element/object-image-only-for-print.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Test print result of image not displayed on screen</title>

<link rel="help" href="https://html.spec.whatwg.org/multipage/iframe-embed-object.html#the-object-element">
<link rel="help" href="https://crbug.com/41477900">

<link rel="match" href="object-image-only-for-print-ref.html">

<style>
  @media not print {
    .print-only {
      display: none;
    }
  }
</style>

<p>
  Should print a green rectangle but not display it on screen.
</p>

<div>
<object
  class="print-only"
  data="/images/green.png"
  type="image/png"
></object>
</div>
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
  "source_name": "html/semantics/embedded-content/the-object-element/object-image-only-for-print.html"
}
```
