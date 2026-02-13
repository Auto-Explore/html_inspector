# html/semantics/forms/the-select-element/inserted-or-removed.html

Counts:
- errors: 0
- warnings: 4
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-select-element/inserted-or-removed.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel="help" href="https://html.spec.whatwg.org/C/#the-select-element:nodes-are-inserted">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<body>

<select id="by-parser">
<option selected>First</option>
<option selected>Second</option>
</select>

<select id="by-parser-optgroup">
<optgroup>
<option selected>First</option>
<option selected>Second</option>
</optgroup>
</select>

<select id="by-dom"></select>

<select id="by-innerHTML"></select>

<script>
test(() => {
  const target = document.querySelector("#by-parser");
  assert_equals(target.selectedOptions[0].textContent, 'Second');

  const target2 = document.querySelector("#by-parser-optgroup");
  assert_equals(target2.selectedOptions[0].textContent, 'Second');
}, 'The last selected OPTION should win; Inserted by parser');

test(() => {
  const target = document.querySelector("#by-dom");
  const option1 = document.createElement('option');
  option1.defaultSelected = true;
  option1.textContent = 'First';
  const option2 = document.createElement('option');
  option2.defaultSelected = true;
  option2.textContent = 'Second';
  target.appendChild(option1);
  target.appendChild(option2);
  assert_equals(target.selectedOptions[0].textContent, 'Second');

  target.innerHTML = '';
  const optgroup = document.createElement('optgroup');
  const option3 = document.createElement('option');
  option3.defaultSelected = true;
  option3.textContent = 'First';
  const option4 = document.createElement('option');
  option4.defaultSelected = true;
  option4.textContent = 'Second';
  optgroup.appendChild(option3);
  optgroup.appendChild(option4);
  target.appendChild(optgroup);
  assert_equals(target.selectedOptions[0].textContent, 'Second');
}, 'The last selected OPTION should win; Inserted by DOM API');

test(() => {
  const target = document.querySelector("#by-dom");
  target.innerHTML = '';
  const inner = `<option value="one" selected>First</option>
      <option value="two" selected>Second</option>`;

  // Emulate what jQuery 1.x/2.x does in append(inner).
  const fragment = document.createDocumentFragment();
  const div = document.createElement('div');
  div.innerHTML = '<select multiple>' + inner + '</select>';
  while (div.firstChild.firstChild)
    fragment.appendChild(div.firstChild.firstChild);
  target.appendChild(fragment);
  assert_equals(target.selectedOptions[0].textContent, 'Second');
}, 'The last selected OPTION should win; Inserted by jQuery append()');

test(() => {
  const target = document.querySelector("#by-innerHTML");
  target.innerHTML = '<option selected>First</option>' +
      '<option selected>Second</option>';
  assert_equals(target.selectedOptions[0].textContent, 'Second');

  target.innerHTML = '<option selected>First</option>' +
      '<optgroup><option selected>Second</option>' +
      '<option selected>Third</option></optgroup>' +
      '<option selected>Fourth</option>';
  assert_equals(target.selectedOptions[0].textContent, 'Fourth');
}, 'The last selected OPTION should win; Inserted by innerHTML');

test (() => {
  for (let insert_location = 0; insert_location < 3; ++insert_location) {
    const target = document.querySelector('#by-innerHTML');
    target.innerHTML = '<option>A</option>' +
        '<option selected>C</option>' +
        '<option>D</option>';
    const refNode = target.querySelectorAll('option')[insert_location];

    const opt = document.createElement('option');
    opt.selected = true;
    opt.textContent = 'B';
    target.insertBefore(opt, refNode);
    assert_equals(target.selectedOptions[0].textContent, 'B');
  }
}, 'If an OPTION says it is selected, it should be selected after it is inserted.');
</script>
</body>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.select.selected.multiple_without_multiple",
      "message": "The “select” element cannot have more than one selected “option” descendant unless the “multiple” attribute is specified.",
      "severity": "Warning",
      "span": {
        "byte_end": 298,
        "byte_start": 281,
        "col": 1,
        "line": 9
      }
    },
    {
      "category": "Html",
      "code": "html.select.selected.multiple_without_multiple",
      "message": "The “select” element cannot have more than one selected “option” descendant unless the “multiple” attribute is specified.",
      "severity": "Warning",
      "span": {
        "byte_end": 418,
        "byte_start": 401,
        "col": 1,
        "line": 15
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
  "source_name": "html/semantics/forms/the-select-element/inserted-or-removed.html"
}
```
