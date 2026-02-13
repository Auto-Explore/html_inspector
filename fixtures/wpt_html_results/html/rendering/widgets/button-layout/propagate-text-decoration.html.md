# html/rendering/widgets/button-layout/propagate-text-decoration.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/widgets/button-layout/propagate-text-decoration.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>propagating text-decoration into buttons</title>
<link rel=match href=propagate-text-decoration-ref.html>
<meta name=fuzzy content="maxDifference=0-40;totalPixels=0-50">
<style>
button, input { font: inherit }
.inline > u > * { display: inline }
.inline-block > u > * { display: inline-block }
</style>
<p>The text in the following buttons should be underlined.</p>
<p class=inline><u><button>foo</button><input type=button value=foo></u></p>
<p>The text in the following buttons should NOT be underlined.</p>
<p class=inline-block><u><button>foo</button><input type=button value=foo></u></p>
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
  "source_name": "html/rendering/widgets/button-layout/propagate-text-decoration.html"
}
```
