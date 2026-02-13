# html/semantics/forms/the-select-element/select-value-option.tentative.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-select-element/select-value-option.tentative.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel=author href="mailto:jarhar@chromium.org">
<link rel=help href="https://github.com/openui/open-ui/issues/664">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<select id=select>
  <option id=optone>innertext one</option>
  <option id=opttwo value=valueattribute>innertext two</option>
</select>

<style>
  select, ::picker(select) {
    appearance: base-select;
  }
</style>

<script>
test(() => {
  assert_equals(select.value, 'innertext one',
    'The first option should be selected initially.');
  select.value = 'valueattribute';
  assert_equals(select.value, 'valueattribute',
    'Assigning value should look at the options value, not innertext');
}, 'select.value should reflect option.value');
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
        "byte_end": 387,
        "byte_start": 380,
        "col": 1,
        "line": 12
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
  "source_name": "html/semantics/forms/the-select-element/select-value-option.tentative.html"
}
```
