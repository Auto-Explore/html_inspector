# html/editing/the-hidden-attribute/hidden-idl.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/editing/the-hidden-attribute/hidden-idl.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel=author href="mailto:jarhar@chromium.org">
<link rel=help href="https://github.com/whatwg/html/pull/7475">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<div>hello</div>
<script>
const div = document.querySelector('div');

function runPropertyTest(assignedValue, expectedValue, expectedAttribute) {
  test(() => {
    div.hidden = assignedValue;
    assert_equals(div.hidden, expectedValue,
      `div.hidden = ${JSON.stringify(assignedValue)} should return ${JSON.stringify(expectedValue)}`);
    assert_equals(div.getAttribute('hidden'), expectedAttribute,
      `div.hidden = ${JSON.stringify(assignedValue)} should set the hidden attribute to ${JSON.stringify(expectedAttribute)}`);
  }, `div.hidden = ${Number.isNaN(assignedValue) ? 'NaN' : JSON.stringify(assignedValue)}`);
}

function runAttributeTest(assignedAttribute, expectedValue) {
  test(() => {
    div.setAttribute('hidden', assignedAttribute);
    assert_equals(div.hidden, expectedValue);
  }, `div.setAttribute('hidden', ${JSON.stringify(assignedAttribute)}) should make div.hidden return ${JSON.stringify(expectedValue)}`);
}

runPropertyTest(false, false, null);
runPropertyTest(true, true, '');
runPropertyTest('foo', true, '');
runPropertyTest('false', true, '');
runPropertyTest('', false, null);

runAttributeTest('false', true);
runAttributeTest('foo', true);

runPropertyTest('until-found', 'until-found', 'until-found');
runPropertyTest('UNTIL-FOUND', 'until-found', 'until-found');
runPropertyTest('UnTiL-FoUnD', 'until-found', 'until-found');
runPropertyTest('unt\u0131l-found', true, '');
runPropertyTest('unt\u0130l-found', true, '');

runPropertyTest(null, false, null);
runPropertyTest(undefined, false, null);

runPropertyTest(1, true, '');
runPropertyTest(0, false, null);
runPropertyTest(NaN, false, null);
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
  "source_name": "html/editing/the-hidden-attribute/hidden-idl.html"
}
```
