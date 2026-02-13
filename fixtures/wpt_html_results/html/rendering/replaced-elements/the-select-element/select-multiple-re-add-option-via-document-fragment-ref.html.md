# html/rendering/replaced-elements/the-select-element/select-multiple-re-add-option-via-document-fragment-ref.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/replaced-elements/the-select-element/select-multiple-re-add-option-via-document-fragment-ref.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>Reference: Move option from select[multiple] into DocumentFragment and back</title>
<p>You should see the word PASS below.</p>
<select multiple><option>PASS</option></select>
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
  "source_name": "html/rendering/replaced-elements/the-select-element/select-multiple-re-add-option-via-document-fragment-ref.html"
}
```
