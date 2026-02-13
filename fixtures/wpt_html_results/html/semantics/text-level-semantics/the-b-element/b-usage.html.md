# html/semantics/text-level-semantics/the-b-element/b-usage.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/text-level-semantics/the-b-element/b-usage.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>HTML test: b - highlight keywords</title>
<link rel="author" title="Intel" href="http://www.intel.com/">
<link rel="mismatch" href="b-usage-notref.html">
<link rel="help" href="https://html.spec.whatwg.org/multipage/text-level-semantics.html#the-b-element"/>

<p>You enter a small room. Your <b>sword</b> glows brighter. A <b>rat</b> scurries past the corner wall.</p>
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
  "source_name": "html/semantics/text-level-semantics/the-b-element/b-usage.html"
}
```
