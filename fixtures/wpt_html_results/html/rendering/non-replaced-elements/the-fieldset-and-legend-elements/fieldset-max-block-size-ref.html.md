# html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/fieldset-max-block-size-ref.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/fieldset-max-block-size-ref.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<style>
.fieldset {
  border: 2px solid gray;
  margin: 1em;
  padding: 0;
  width: 20em;
}

.f1 {
  overflow: auto;
  max-height: 3em;
}
.f2 {
  max-height: 0;
}
</style>

<div class="fieldset f1">
<div>foo</div>
<div>foo</div>
<div>foo</div>
<div>foo</div>
<div>foo</div>
</div>

<div class="fieldset f1">
<div>foo</div>
</div>

<div class="fieldset f2">
<div>foo</div>
</div>

```

```json
{
  "messages": [
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
  "source_name": "html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/fieldset-max-block-size-ref.html"
}
```
