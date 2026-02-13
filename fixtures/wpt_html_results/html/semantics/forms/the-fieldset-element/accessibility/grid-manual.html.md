# html/semantics/forms/the-fieldset-element/accessibility/grid-manual.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-fieldset-element/accessibility/grid-manual.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>fieldset accessibility test: grid</title>
<link rel=help href=https://w3c.github.io/html-aam/#fieldset-element-accessible-name-computation>
<style>
 fieldset { display: grid; grid-template-columns: auto auto; }
 legend { float: left; }
</style>
<fieldset id=fieldset>
 <legend>Foo</legend>
 <input>
</fieldset>
<p>Expected accessible name for id=fieldset: "Foo"
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
  "source_name": "html/semantics/forms/the-fieldset-element/accessibility/grid-manual.html"
}
```
