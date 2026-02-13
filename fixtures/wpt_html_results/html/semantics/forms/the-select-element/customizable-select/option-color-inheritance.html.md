# html/semantics/forms/the-select-element/customizable-select/option-color-inheritance.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-select-element/customizable-select/option-color-inheritance.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel=author href="mailto:jarhar@chromium.org">
<link rel=help href="https://issues.chromium.org/issues/404464591">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<select>
  <option>option</option>
</select>
<style>
select {
  color: rgb(12, 34, 56);
}
</style>

<script>
test(() => {
  const style = getComputedStyle(document.querySelector('option'));
  assert_equals(style.color, 'rgb(12, 34, 56)', 'color');
}, 'option elements should inherit color from their parent select element.');
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.style.not_allowed_here",
      "message": "Element “style” not allowed as child of “body” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 295,
        "byte_start": 288,
        "col": 1,
        "line": 10
      }
    },
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
  "source_name": "html/semantics/forms/the-select-element/customizable-select/option-color-inheritance.html"
}
```
