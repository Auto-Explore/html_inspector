# html/semantics/forms/the-select-element/customizable-select/option-disabled-optgroup.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-select-element/customizable-select/option-disabled-optgroup.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel=author href="mailto:jarhar@chromium.org">
<link rel=help href="https://html.spec.whatwg.org/multipage/form-elements.html#concept-option-disabled">
<link rel=help href="https://github.com/whatwg/html/issues/11707">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<style>
  optgroup { color: black }
  option { color: black }
  option:disabled { color: gray }
</style>

<select>
  <optgroup>
    <div>
      <option>What color is this?</option>
    </div>
  <optgroup>
</select>

<script>
test(() => {
  const optgroup = document.querySelector('optgroup');
  const option = document.querySelector('option');
  const optionComputedStyle = getComputedStyle(option);

  assert_equals(optionComputedStyle.color, 'rgb(0, 0, 0)',
    'color before optgroup disabled');

  optgroup.disabled = true;
  assert_equals(optionComputedStyle.color, 'rgb(128, 128, 128)',
    'color after optgroup disabled');
}, 'options should check ancestor optgroup for disabled state.');

test(() => {
  const parentOptgroup = document.createElement('optgroup');
  const childOptgroup = document.createElement('optgroup');
  parentOptgroup.appendChild(childOptgroup);
  const option = document.createElement('option');
  childOptgroup.appendChild(option);

  parentOptgroup.disabled = true;
  assert_false(option.matches(':disabled'));
}, 'nested optgroup');

test(() => {
  const select = document.createElement('select');
  const option = document.createElement('option');
  select.appendChild(option);
  select.disabled = true;

  assert_false(option.matches(':disabled'));
}, 'disabled select');

test(() => {
  const optgroup = document.createElement('optgroup');
  const select = document.createElement('select');
  optgroup.appendChild(select);
  const option = document.createElement('option');
  select.appendChild(option);

  optgroup.disabled = true;
  assert_false(option.matches(':disabled'));
}, 'select in optgroup');

test(() => {
  const optgroup = document.createElement('optgroup');
  const parentOption = document.createElement('option');
  optgroup.appendChild(parentOption);
  const childOption = document.createElement('option');
  parentOption.appendChild(childOption);

  optgroup.disabled = true;
  assert_true(parentOption.matches(':disabled'), 'parent option');
  assert_false(childOption.matches(':disabled'), 'child option');
}, 'nested options');
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
  "source_name": "html/semantics/forms/the-select-element/customizable-select/option-disabled-optgroup.html"
}
```
