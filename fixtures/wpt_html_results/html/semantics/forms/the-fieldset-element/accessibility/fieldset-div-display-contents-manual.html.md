# html/semantics/forms/the-fieldset-element/accessibility/fieldset-div-display-contents-manual.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-fieldset-element/accessibility/fieldset-div-display-contents-manual.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>fieldset accessibility test: fieldset div display: contents</title>
<link rel=help href=https://w3c.github.io/html-aam/#fieldset-element-accessible-name-computation>
<style>
 div { display: contents; }
</style>
<fieldset id=fieldset>
 <div>
  <legend>Foo</legend>
  <input>
 </div>
</fieldset>
<p>Expected accessible name for id=fieldset: ""
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
  "source_name": "html/semantics/forms/the-fieldset-element/accessibility/fieldset-div-display-contents-manual.html"
}
```
