# html/semantics/forms/the-meter-element/meter-min-rendering.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-meter-element/meter-min-rendering.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset=utf-8>
<title>min is accounted for when rendering a &lt;meter&gt; element</title>
<link rel=author href="mailto:emilio@crisal.io" title="Emilio Cobos Álvarez">
<link rel=author href="https://mozilla.org" title="Mozilla">
<link rel=help href="https://bugzilla.mozilla.org/show_bug.cgi?id=1746758">
<link rel=match href="meter-min-rendering-ref.html">

<meter min="1.0" max="2.0" value="1.5">
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
  "source_name": "html/semantics/forms/the-meter-element/meter-min-rendering.html"
}
```
