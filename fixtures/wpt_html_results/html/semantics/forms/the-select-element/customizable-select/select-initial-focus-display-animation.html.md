# html/semantics/forms/the-select-element/customizable-select/select-initial-focus-display-animation.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-select-element/customizable-select/select-initial-focus-display-animation.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel=author href="mailto:jarhar@chromium.org">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-vendor.js"></script>
<script src="/resources/testdriver-actions.js"></script>

<select>
  <option>option</option>
</select>

<style>
select, ::picker(select) {
  appearance: base-select;
}

::picker(select) {
  transition: display allow-discrete 1s, opacity 1s;
}

select:not(:open)::picker(select) {
  opacity: 0;
}
</style>

<script>
const select = document.querySelector('select');
const option = document.querySelector('option');
promise_test(async () => {
  await test_driver.click(select);
  await new Promise(requestAnimationFrame);
  await new Promise(requestAnimationFrame);
  assert_equals(document.activeElement, option);
}, 'Option initial focus should still work when a display animation is present.');
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
        "byte_end": 390,
        "byte_start": 383,
        "col": 1,
        "line": 13
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
  "source_name": "html/semantics/forms/the-select-element/customizable-select/select-initial-focus-display-animation.html"
}
```
