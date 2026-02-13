# html/semantics/forms/the-select-element/customizable-select/option-list.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-select-element/customizable-select/option-list.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel=author href="mailto:jarhar@chromium.org">
<link rel=help href="https://github.com/whatwg/html/pull/10557">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<script>
// Structure:
// <select id=parentSelect>
//   <select id=childSelect>
//     <option id=normalOption>
//       <option id=nestedOption>
//     <div>
//       <option id=divOption>
//     <hr>
//       <option id=hrOption>
//     <datalist>
//       <option id=datalistOption>
//     <optgroup>
//       <option id=optgroupOption>
//       <optgroup id=nestedOptgroup>
//         <option id=nestedOptgroupOption>
const parentSelect = document.createElement('select');
const childSelect = document.createElement('select');
parentSelect.appendChild(childSelect);
const normalOption = document.createElement('option');
childSelect.appendChild(normalOption);
const nestedOption = document.createElement('option');
normalOption.appendChild(nestedOption);
const div = document.createElement('div');
childSelect.appendChild(div);
const divOption = document.createElement('option');
div.appendChild(divOption);
const hr = document.createElement('hr');
childSelect.appendChild(hr);
const hrOption = document.createElement('option');
hr.appendChild(hrOption);
const datalist = document.createElement('datalist');
childSelect.appendChild(datalist);
const datalistOption = document.createElement('option');
datalist.appendChild(datalistOption);
const optgroup = document.createElement('optgroup');
childSelect.appendChild(optgroup);
const optgroupOption = document.createElement('option');
optgroup.appendChild(optgroupOption);
const nestedOptgroup = document.createElement('optgroup');
optgroup.appendChild(nestedOptgroup);
const nestedOptgroupOption = document.createElement('option');
nestedOptgroup.appendChild(nestedOptgroupOption);

test(() => {
  const expectedOptions = [
    normalOption,
    divOption,
    optgroupOption
  ];
  assert_equals(parentSelect.length, 0, 'parentSelect.length');
  assert_equals(parentSelect.options.length, 0, 'parentSelect.options.length');
  assert_equals(childSelect.length, expectedOptions.length, 'childSelect.length');
  assert_equals(childSelect.options.length, expectedOptions.length, 'childSelect.options.length');
  for (let i = 0; i < expectedOptions.length; i++) {
    assert_equals(childSelect.options[i], expectedOptions[i], `option ${i}.`);
  }
}, `select's option list should not include descendants of options, hrs, or nested optgroups.`);
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
  "source_name": "html/semantics/forms/the-select-element/customizable-select/option-list.html"
}
```
