# html/rendering/non-replaced-elements/tables/table-ua-stylesheet.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/tables/table-ua-stylesheet.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>Test for table element's UA-stylesheet-provided styles</title>
<link rel="help" href="https://html.spec.whatwg.org/multipage/rendering.html#the-css-user-agent-style-sheet-and-presentational-hints">
<link rel="help" href="https://html.spec.whatwg.org/multipage/rendering.html#tables-2">
<link rel="author" title="Daniel Holbert" href="mailto:dholbert@mozilla.com">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<div id="refElem"></div>
<!-- Note: this test puts the table inside of an element with a non-default
     'text-indent' and 'border-collapse' values, so that we can verify that
     the table does indeed use the initial value for these properties, rather
     than simply inheriting. -->
<div style="text-indent: 100px; border-collapse: collapse">
  <table id="tableElem"></table>
</div>

<script>
/* These styles come from the default `table` styling here:
 *  https://html.spec.whatwg.org/multipage/rendering.html#tables-2
 * We can't check for these values directly, because they may be
 * serialized slightly differently when read from the computed style.
 * So, for each property here, we apply it to a "reference" div and then
 * read back the computed value, and we validate that a table element
 * has that same computed value by default. */
const defaultTablePropVals = {
  'display':         'table',
  'box-sizing':      'border-box',
  'border-spacing':  '2px',
  'border-collapse': 'separate',
  'text-indent':     'initial',
};

for (var propName in defaultTablePropVals) {
  test(function() {
    refElem.style[propName] = defaultTablePropVals[propName];
    let expectedComputedVal = getComputedStyle(refElem, "")[propName];

    let actualComputedVal = getComputedStyle(tableElem, "")[propName];
    assert_equals(actualComputedVal, expectedComputedVal);
  }, `Computed '${propName}' on table should match html spec`);
}
</script>
```

```json
{
  "messages": [
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/rendering/non-replaced-elements/tables/table-ua-stylesheet.html"
}
```
