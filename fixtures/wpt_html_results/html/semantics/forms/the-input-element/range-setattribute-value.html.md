# html/semantics/forms/the-input-element/range-setattribute-value.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-input-element/range-setattribute-value.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html class="reftest-wait">
<meta charset="utf-8">
<link rel="author" title="Joey Arhar" href="mailto:jarhar@chromium.org">
<link rel="help" href="https://html.spec.whatwg.org/multipage/input.html#the-input-element">
<link rel="match" href="range-setattribute-value-ref.html">
<title>range input element setAttribute value appearance</title>

<p>Test passes if the range element below visually has its slider at 2/10 from the left</p>

<script>
window.onload = () => {

  const input = document.createElement('input');
  input.type = 'range';
  input.min = 0;
  input.max = 10;
  document.body.appendChild(input);

  requestAnimationFrame(() => {
    requestAnimationFrame(() => {
      input.setAttribute('value', 2);
      document.documentElement.classList.remove('reftest-wait');
    });
  });
};
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
  "source_name": "html/semantics/forms/the-input-element/range-setattribute-value.html"
}
```
