# html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/fieldset-div-display-contents.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/fieldset-div-display-contents.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>fieldset and div with display: contents</title>
<link rel=fieldset-foo-ref.html>
<style>
div { display: contents; }
</style>
<p>There should be a normal fieldset below with the legend "Foo".</p>
<fieldset>
 <div>
  <legend>Foo</legend>
 </div>
</fieldset>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.link.href.required",
      "message": "A “link” element must have an “href” or “imagesrcset” attribute, or both.",
      "severity": "Warning",
      "span": {
        "byte_end": 103,
        "byte_start": 71,
        "col": 1,
        "line": 3
      }
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/fieldset-div-display-contents.html"
}
```
