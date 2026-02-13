# html/semantics/forms/the-fieldset-element/accessibility/fieldset-role-none-manual.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-fieldset-element/accessibility/fieldset-role-none-manual.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>fieldset accessibility test: fieldset role=none</title>
<link rel=help href=http://w3c.github.io/aria/#none>
<fieldset id=fieldset role=none>
 <legend>Foo</legend>
 <input>
</fieldset>
<p>Expected no accessible node for id=fieldset.
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
  "source_name": "html/semantics/forms/the-fieldset-element/accessibility/fieldset-role-none-manual.html"
}
```
