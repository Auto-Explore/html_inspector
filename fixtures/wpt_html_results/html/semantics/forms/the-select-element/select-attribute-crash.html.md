# html/semantics/forms/the-select-element/select-attribute-crash.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-select-element/select-attribute-crash.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html class=test-wait>
<link rel=author href="mailto:jarhar@chromium.org">
<link rel=help href="https://issues.chromium.org/324325525">

<!-- Attempting to slot a child of <select> after initial slot assignment should not crash. -->

<select size=1>
  <optgroup></optgroup>
</select>
<script>
requestAnimationFrame(() => {
  document.querySelector('optgroup').setAttribute('slot', 'slot1');
  requestAnimationFrame(() => {
    document.documentElement.classList.remove('test-wait');
  });
});
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.head.title.missing",
      "message": "Element “head” is missing a required instance of child element “title”.",
      "severity": "Warning",
      "span": null
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/semantics/forms/the-select-element/select-attribute-crash.html"
}
```
