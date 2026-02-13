# html/semantics/forms/the-select-element/customizable-select/option-form-ancestor-select.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-select-element/customizable-select/option-form-ancestor-select.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel=author href="mailto:jarhar@chromium.org">
<link rel=help href="https://html.spec.whatwg.org/multipage/form-elements.html#dom-option-form">
<link rel=help href="https://github.com/whatwg/html/issues/11708">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<form id=form>
  <select id=select1>
    <optgroup></optgroup>
  </select>
</form>

<select id=select2 form=form>
  <optgroup></optgroup>
</select>

<script>
const form = document.getElementById('form');
const select1 = document.getElementById('select1');
const select2 = document.getElementById('select2');

test(() => {
  const div1 = document.createElement('div');
  select1.appendChild(div1);
  const option1 = document.createElement('option');
  div1.appendChild(option1);

  const div2 = document.createElement('div');
  select2.appendChild(div2);
  const option2 = document.createElement('option');
  div2.appendChild(option2);

  assert_equals(option1.form, form, 'option1');
  assert_equals(option2.form, form, 'option2');
}, 'option.form should look up the ancestor chain for a select element to get its form from.');

test(() => {
  ['datalist', 'optgroup', 'datalist', 'hr'].forEach(tagName => {
    ['select1', 'select2'].forEach(selectId => {
      const select = document.getElementById(selectId);
      const container = select.querySelector('optgroup');
      const blockingElement = document.createElement(tagName);
      container.appendChild(blockingElement);
      const option = document.createElement('option');
      blockingElement.appendChild(option);
      assert_equals(option.form, null, `${selectId} ${tagName}: option.form should be null.`);
    });
  });
}, 'option.form should be null when options are nested in invalid elements.');
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
  "source_name": "html/semantics/forms/the-select-element/customizable-select/option-form-ancestor-select.html"
}
```
