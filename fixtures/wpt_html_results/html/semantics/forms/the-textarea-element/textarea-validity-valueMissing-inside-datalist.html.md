# html/semantics/forms/the-textarea-element/textarea-validity-valueMissing-inside-datalist.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-textarea-element/textarea-validity-valueMissing-inside-datalist.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset="utf-8">
<title>textarea element with "required" attribute and empty value is considered "suffering from being missing" even if inside datalist element</title>
<link rel="help" href="https://html.spec.whatwg.org/multipage/form-elements.html#the-textarea-element:suffering-from-being-missing">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<datalist>
  <textarea required></textarea>
</datalist>

<script>
test(() => {
  const textarea = document.querySelector("textarea");

  assert_true(textarea.validity.valueMissing);
  assert_false(textarea.validity.valid);
});
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
  "source_name": "html/semantics/forms/the-textarea-element/textarea-validity-valueMissing-inside-datalist.html"
}
```
