# html/semantics/forms/the-form-element/form-elements-sameobject.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-form-element/form-elements-sameobject.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset="utf-8">
<title>Testing [SameObject] on the 'elements' attribute on the 'form' element</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id="log"></div>

<form>
  <input>
</form>

<script>
test(function() {
  var form = document.querySelector('form');
  var elements = form.elements;
  assert_equals(elements, form.elements);
  form.appendChild(document.createElement('input'));
  assert_equals(elements, form.elements);
}, "[SameObject] should apply to 'elements' attr on <form>");
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
  "source_name": "html/semantics/forms/the-form-element/form-elements-sameobject.html"
}
```
