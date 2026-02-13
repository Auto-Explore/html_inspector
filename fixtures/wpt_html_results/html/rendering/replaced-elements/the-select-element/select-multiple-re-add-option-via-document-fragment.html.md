# html/rendering/replaced-elements/the-select-element/select-multiple-re-add-option-via-document-fragment.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/replaced-elements/the-select-element/select-multiple-re-add-option-via-document-fragment.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>Test: Move option from select[multiple] into DocumentFragment and back</title>
<link rel="help" href="https://html.spec.whatwg.org/multipage/rendering.html#the-select-element-2">
<link rel="match" href="select-multiple-re-add-option-via-document-fragment-ref.html">
<p>You should see the word PASS below.</p>
<select multiple id="sel"><option id="opt">PASS</option></select>
<script>
  document.body.offsetTop;
  let rm = opt;
  document.createDocumentFragment().appendChild(rm);
  sel.appendChild(rm);
</script>
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
  "source_name": "html/rendering/replaced-elements/the-select-element/select-multiple-re-add-option-via-document-fragment.html"
}
```
