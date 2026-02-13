# html/rendering/widgets/button-layout/display-other.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/widgets/button-layout/display-other.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>button with other display values</title>
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>
<style>
body { margin: 0 }
.float { width: 100px; height: 100px; float: left; background: blue; margin: 10px }
</style>
<div class=float></div>
<button id=button style="display: block;"><div class=float></div></button><span id=after>after</span>
<script>
// These should all behave as flow-root.
const displayValues = ['run-in', 'flow', 'flow-root', 'table', 'list-item',
                       'table-row-group', 'table-header-group', 'table-footer-group',
                       'table-row', 'table-cell', 'table-column-group', 'table-column',
                       'table-caption', 'ruby-base', 'ruby-text', 'ruby-base-container',
                       'ruby-text-container'];
const button = document.getElementById('button');
const after = document.getElementById('after');
function getValues() {
  return {
    buttonLeft: button.offsetLeft,
    buttonTop: button.offsetTop,
    buttonWidth: button.clientWidth,
    buttonHeight: button.clientHeight,
    afterLeft: after.offsetLeft,
    afterTop: after.offsetTop,
  };
}
const expected = getValues();
test(t => {
  assert_equals(expected.buttonLeft, 120, 'buttonLeft');
  assert_equals(expected.buttonTop, 0, 'buttonTop');
  assert_greater_than_equal(expected.buttonWidth, 120, 'buttonWidth');
  assert_greater_than_equal(expected.buttonHeight, 120, 'buttonHeight');
  assert_equals(expected.afterLeft, 0, 'afterLeft');
  assert_greater_than_equal(expected.afterTop, 120, 'afterTop');
}, 'display: block');
for (const val of displayValues) {
  test(t => {
    t.add_cleanup(() => {
      button.style.display = 'block';
    });
    assert_true(CSS.supports(`display: ${val}`), `display: ${val} is not supported`);
    button.style.display = val;
    const values = getValues();
    for (const prop in values) {
      assert_equals(values[prop], expected[prop], prop);
    }
  }, `display: ${val}`);
}
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
        "byte_end": 369,
        "byte_start": 352,
        "col": 43,
        "line": 10
      }
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/rendering/widgets/button-layout/display-other.html"
}
```
