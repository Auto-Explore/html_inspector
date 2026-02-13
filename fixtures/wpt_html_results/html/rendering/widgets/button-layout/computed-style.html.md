# html/rendering/widgets/button-layout/computed-style.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/widgets/button-layout/computed-style.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>computed style on buttons</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div class="tests">
 <input type="reset">
 <input type="button">
 <input type="submit">
 <input type="color">
 <input type="image">
 <button></button>
</div>
<script>
// "behave as" doesn't change computed value.
const displayValues = ['inline', 'block', 'list-item', 'inline-block', 'table', 'inline-table', 'table-row-group', 'table-header-group', 'table-footer-group', 'table-row', 'table-column-group', 'table-column', 'table-cell', 'table-caption', 'none', 'contents', 'flow', 'flow-root', 'flex', 'grid', 'ruby', 'ruby-base', 'ruby-text', 'ruby-base-container', 'ruby-text-container', 'inline-flex', 'inline-grid'];
for (const val of displayValues) {
  [].slice.call(document.querySelectorAll('.tests > *')).forEach(el => {
    el.style.display = ''
    el.style.display = val;
    const attrs = el.type ? ` type=${el.type}` : '';
    const tag = `<${el.localName}${attrs}>`;
    test(() => {
     assert_not_equals(el.style.display, '', `display: ${val} is not supported`)
      let expectedVal = val;
      if (el instanceof HTMLInputElement && val === 'contents') {
        expectedVal = 'none'; // https://drafts.csswg.org/css-display/#unbox-html
      }
      if (val == 'flow') {
        // Use the more backwards-compatible form, `block` is better than `flow`
        // https://drafts.csswg.org/cssom/#serializing-css-values
        expectedVal = 'block';
      }
      assert_equals(getComputedStyle(el).display, expectedVal);
    }, `computed display of ${tag} with display: ${val}`);
  });
}

for (let input of document.querySelectorAll("input")) {
  test(() => {
    if (input.type == "image") {
      assert_equals(getComputedStyle(input).overflow, "visible", "Should not be clip by default");
      return;
    }
    assert_equals(getComputedStyle(input).overflow, "clip", "Should be clip by default");
    assert_equals(getComputedStyle(input).overflowClipMargin, "0px", "Should use 0 margin");
    input.style.overflow = "visible";
    input.style.overflowClipMargin = "10px";
    assert_equals(getComputedStyle(input).overflow, "clip", "Clip be !important");
    assert_equals(getComputedStyle(input).overflowClipMargin, "0px", "Clip margin should be !important too");
  }, `<input type=${input.type}> overflow/overflow-clip-margin`);
}
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.input.button.value.nonempty",
      "message": "Element “input” with attribute “type” whose value is “button” must have non-empty attribute “value”.",
      "severity": "Warning",
      "span": {
        "byte_end": 227,
        "byte_start": 206,
        "col": 2,
        "line": 7
      }
    },
    {
      "category": "Html",
      "code": "html.input.image.alt.missing",
      "message": "Element “input” is missing required attribute “alt”.",
      "severity": "Warning",
      "span": {
        "byte_end": 294,
        "byte_start": 274,
        "col": 2,
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
  "source_name": "html/rendering/widgets/button-layout/computed-style.html"
}
```
