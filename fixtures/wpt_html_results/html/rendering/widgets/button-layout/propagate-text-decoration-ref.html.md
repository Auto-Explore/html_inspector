# html/rendering/widgets/button-layout/propagate-text-decoration-ref.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/widgets/button-layout/propagate-text-decoration-ref.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Reference for propagating text-decoration into buttons</title>
<style>
button { font: inherit }
</style>
<p>The text in the following buttons should be underlined.</p>
<p><button><u>foo</u></button><button><u>foo</u></button></p>
<p>The text in the following buttons should NOT be underlined.</p>
<p><button>foo</button><button>foo</button></p>
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
  "source_name": "html/rendering/widgets/button-layout/propagate-text-decoration-ref.html"
}
```
