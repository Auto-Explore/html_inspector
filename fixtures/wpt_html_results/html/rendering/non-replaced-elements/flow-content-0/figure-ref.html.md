# html/rendering/non-replaced-elements/flow-content-0/figure-ref.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/flow-content-0/figure-ref.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>The figure element</title>
<link rel=author title=Ms2ger href=ms2ger@gmail.com>
<link rel=help href=https://html.spec.whatwg.org/multipage/#the-figure-element>
<style>
body > div { margin: 1em 40px; }
</style>
<div>
<div>Caption</div>
Figure
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
  "source_name": "html/rendering/non-replaced-elements/flow-content-0/figure-ref.html"
}
```
