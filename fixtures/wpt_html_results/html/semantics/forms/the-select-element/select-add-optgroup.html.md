# html/semantics/forms/the-select-element/select-add-optgroup.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-select-element/select-add-optgroup.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel=author href="mailto:jarhar@chromium.org">
<link rel=help href="https://bugs.chromium.org/p/chromium/issues/detail?id=1477785">
<link rel=help href="https://html.spec.whatwg.org/multipage/common-dom-interfaces.html#dom-htmloptionscollection-add">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<select>
  <option id=opt1>opt1</option>
  <optgroup label=group1>
    <option id=opt2>opt2</option>
  </optgroup>
</select>

<script>
test(() => {
  const select = document.querySelector('select');
  const optgroup = document.querySelector('optgroup');
  const newOption = document.createElement('option');
  newOption.textContent = 'new option';

  select.options.add(newOption, 1);
  assert_equals(select.options.length, 3);
  assert_equals(select.options[0], opt1, 'First item should be opt1.');
  assert_equals(select.options[1], newOption, 'Second item should be newOption.');
  assert_equals(select.options[2], opt2, 'Third item should be opt2.');
  assert_equals(newOption.parentNode, optgroup, 'The new option should be inside the optgroup.');
}, 'select.add() with an index should work when the target is inside an optgroup.');
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
  "source_name": "html/semantics/forms/the-select-element/select-add-optgroup.html"
}
```
