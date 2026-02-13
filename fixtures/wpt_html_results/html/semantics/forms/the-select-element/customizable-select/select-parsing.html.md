# html/semantics/forms/the-select-element/customizable-select/select-parsing.html

Counts:
- errors: 0
- warnings: 6
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-select-element/customizable-select/select-parsing.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel=author href="mailto:jarhar@chromium.org">
<link rel=help href="https://github.com/whatwg/html/issues/9799">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<body>

<select class=test
    data-description='<div>s, <button>s, and <datalist>s should be allowed in <select>'
    data-expect='
      <div>div 1</div>
      <button>button</button>
      <div>div 2</div>
      <datalist>
        <option>option</option>
      </datalist>
      <div>div 3</div>
      '>
  <div>div 1</div>
  <button>button</button>
  <div>div 2</div>
  <datalist>
    <option>option</option>
  </datalist>
  <div>div 3</div>
</select>

<select class=test
    data-description='</select> should close <button>'
    data-expect='<button>button</button>'>
  <button>button
</select>

<select class=test
    data-description='</select> should close <datalist>'
    data-expect='<datalist>datalist</datalist>'>
  <datalist>datalist
</select>

<select id=nested1 class=test
    data-description='<select> in <button> in <select> should remove inner <select>'
    data-expect='<button></button>'>
  <button>
    <select id=expectafter1></select>
    <div id=expectafter1b></div>
  </button>
</select>

<select id=nested2 class=test
    data-description='<select> in <select><button><div> should remove inner <select>'
    data-expect='<button><div></div></button>'>
  <button>
    <div>
      <select id=expectafter2>
</select>

<select
  id=nested3
  class=test
  data-description='JS added nested <select> should be ignored'
  data-expect='<option>The Initial Option</option>'
>
  <option>The Initial Option</option>
</select>

<select
  id=nested4
  class=test
  data-description='JS added nested <select>s should be ignored'
  data-expect='<option>The Initial Option</option>'
>
  <option>The Initial Option</option>
</select>

<select class=test
    data-description='Divs and imgs should be allowed as direct children of select and within options without a datalist'
    data-expect='
    <div>
      <option><img>option</option>
    </div>'>
  <div>
    <option><img>option</option>
  </div>
</select>

<select class=test
    data-description='Input tags should not parse inside select instead of closing the select'
    data-expect=''>
    <input>
</select>

<select class=test
    data-description='textarea tags should parse inside select instead of closing the select'
    data-expect='<textarea></textarea>'>
    <textarea></textarea>
</select>

<select class=test
    data-description='Input tags should parse inside select if nested in another tag'
    data-expect='<div><input></div>'>
  <div>
    <input>
  </div>
</select>

<select class=test
        data-description='Input tags should close select when directly inside an <option>'
        data-expect='<option></option>'>
  <option>
    <input>
  </option>
</select>

<div id=afterlast>
  keep this div after the last test case
</div>

<script>
function removeWhitespace(t) {
  return t.replace(/\s/g,'');
}
document.querySelectorAll('select.test').forEach(s => {
  assert_true(!!s.dataset.description.length);
  test(() => {
    // The document.body check here and in the other tests is to make sure that a
    // previous test case didn't leave the HTML parser open on another element.
    assert_equals(s.parentNode, document.body);
    assert_equals(removeWhitespace(s.innerHTML),removeWhitespace(s.dataset.expect));
  },s.dataset.description)
});

test(() => {
  assert_equals(document.getElementById('afterlast').parentNode, document.body);
}, 'The last test should not leave any tags open after parsing');

test(() => {
  const outerSelect = document.getElementById('nested1');
  const innerSelect = document.getElementById('expectafter1');
  const nextDiv = document.getElementById('expectafter1b');
  assert_true(!!outerSelect);
  assert_equals(innerSelect, null,'Nested select should be removed');
  assert_equals(outerSelect.nextElementSibling, nextDiv,'Subsequent content is there too');
}, 'Nested selects should be retained 1');

test(() => {
  const outerSelect = document.getElementById('nested2');
  const innerSelect = document.getElementById('expectafter2');
  assert_true(!!outerSelect);
  assert_equals(innerSelect, null,'Nested select should be pushed out as the next sibling');
}, 'Nested selects should be retained 2');

test(() => {
  assert_true(!!nested3);
  nested3.innerHTML = '<select id="ignored"><option>The New Option</option></select>';

  const ignored = document.getElementById('ignored');
  assert_equals(ignored, null);

  assert_equals(nested3.innerHTML, '<option>The New Option</option>');
}, 'JS added nested select should be ignored');

test(() => {
  assert_true(!!nested4);
  nested4.innerHTML = '<select id="ignore1"><select id="ignore2"><option>The New Option</option></select></select>';

  const ignored1 = document.getElementById('ignored1');
  assert_equals(ignored1, null);
  const ignored2 = document.getElementById('ignored2');
  assert_equals(ignored2, null);

  assert_equals(nested4.innerHTML, '<option>The New Option</option>');
}, 'JS added nested selects should be ignored');
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.child.disallowed_in_phrasing_parent",
      "message": "Element “div” not allowed as child of “button” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 1441,
        "byte_start": 1436,
        "col": 5,
        "line": 54
      }
    },
    {
      "category": "Html",
      "code": "html.img.src_or_srcset.missing",
      "message": "Element “img” is missing one or more of the following attributes: “src”, “srcset”.",
      "severity": "Warning",
      "span": {
        "byte_end": 2129,
        "byte_start": 2124,
        "col": 13,
        "line": 83
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 2129,
        "byte_start": 2124,
        "col": 13,
        "line": 83
      }
    },
    {
      "category": "Html",
      "code": "html.option.empty_without_label",
      "message": "Element “option” without attribute “label” must not be empty.",
      "severity": "Warning",
      "span": {
        "byte_end": 2881,
        "byte_start": 2872,
        "col": 3,
        "line": 112
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
  "source_name": "html/semantics/forms/the-select-element/customizable-select/select-parsing.html"
}
```
