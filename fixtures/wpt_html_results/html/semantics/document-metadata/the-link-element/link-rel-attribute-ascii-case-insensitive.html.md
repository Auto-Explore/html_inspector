# html/semantics/document-metadata/the-link-element/link-rel-attribute-ascii-case-insensitive.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/document-metadata/the-link-element/link-rel-attribute-ascii-case-insensitive.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>link element rel is ASCII case-insensitive</title>
<link rel="help" href="https://html.spec.whatwg.org/#the-link-element">
<link rel="help" href="https://html.spec.whatwg.org/#attr-link-rel">
<link rel="help" href="https://html.spec.whatwg.org/#linkTypes">
<meta name="assert" content="link element's rel attribute is ASCII case-insensitive.">
<link rel="mismatch" href="link-rel-attribute-ascii-case-insensitive-notref.html">

<!-- Load sheet with a red background (rel attribute value is case-sensitive
     equal to "stylesheet") -->
<link rel="stylesheet" href="stylesheet.css">

<!-- Load sheet with white background (rel attribute value is ASCII
     case-insensitive equal to "stylesheet") -->
<link rel="StyLeShEeT" href="style.css">

<!-- Do not load sheet with a red background (rel attribute value is
     case-insensitive equal to "stylesheet") -->
<link rel="ſtyleſheet" href="stylesheet.css">

<p>Test passes if background is not red.</p>
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
  "source_name": "html/semantics/document-metadata/the-link-element/link-rel-attribute-ascii-case-insensitive.html"
}
```
