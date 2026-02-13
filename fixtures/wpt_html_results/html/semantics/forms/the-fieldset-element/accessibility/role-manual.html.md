# html/semantics/forms/the-fieldset-element/accessibility/role-manual.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-fieldset-element/accessibility/role-manual.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>fieldset accessibility test: role</title>
<fieldset id=fieldset>
 <legend id=legend>Foo</legend>
 <input>
</fieldset>
<p>Expected accessible role for id=fieldset: "group"
<p>Expected accessible role for id=legend: No corresponding role
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
  "source_name": "html/semantics/forms/the-fieldset-element/accessibility/role-manual.html"
}
```
