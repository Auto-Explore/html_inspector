# html/semantics/forms/the-select-element/customizable-select/option-disabled-invalid-nesting.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-select-element/customizable-select/option-disabled-invalid-nesting.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel=author href="mailto:jarhar@chromium.org">
<link rel=help href="https://github.com/whatwg/html/pull/11790#issuecomment-3404807465">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<select>
  <optgroup disabled>
    <option></option>
    <datalist></datalist>
  </optgroup>
</select>

<script>
test(() => {
  const optionContainer = document.querySelector('option');
  assert_true(optionContainer.matches(':disabled'),
    'Valid option without nesting should be disabled.');

  const nestedInOption = document.createElement('option');
  optionContainer.appendChild(nestedInOption);
  assert_false(nestedInOption.matches(':disabled'),
    'Option nested in option should not inherit disabledness.');

  const parentOptgroup = document.querySelector('optgroup');
  const childOptgroup = document.createElement('optgroup');
  parentOptgroup.appendChild(childOptgroup);
  const nestedInOptgroup = document.createElement('option');
  childOptgroup.appendChild(nestedInOptgroup);
  assert_false(nestedInOptgroup.matches(':disabled'),
    'Option nested in doubly nested optgroups should not inherit disabledness.');

  const hr = document.createElement('hr');
  parentOptgroup.appendChild(hr);
  const nestedInHr = document.createElement('option');
  hr.appendChild(nestedInHr);
  assert_false(nestedInHr.matches(':disabled'),
    'Option nested in hr should not inherit disabledness.');

  const datalist = document.querySelector('datalist');
  const nestedInDatalist = document.createElement('option');
  datalist.appendChild(nestedInDatalist);
  assert_false(nestedInDatalist.matches(':disabled'),
    'Option nested in datalist should not inherit disabledness.');
}, 'options should not inherit disabledness when nested in invalid elements.');
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.option.empty_without_label",
      "message": "Element “option” without attribute “label” must not be empty.",
      "severity": "Warning",
      "span": {
        "byte_end": 316,
        "byte_start": 307,
        "col": 13,
        "line": 9
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
  "source_name": "html/semantics/forms/the-select-element/customizable-select/option-disabled-invalid-nesting.html"
}
```
