# html/semantics/forms/the-legend-element/legend-form.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-legend-element/legend-form.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>HTMLLegendElement Test: form</title>
<link rel="author" title="Intel" href="http://www.intel.com/">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<div id="log"></div>

<div style="display:none">
  <form id="testform">
    <legend id="testlegend">radio</legend>
  </form>
</div>

<div style="display:none">
  <form id="testformWithFieldSet">
    <fieldset>
    <legend id="legendWithFieldSet">radio</legend>
    </fieldset>
  </form>
</div>
<script>
test(function () {
  var legendEle = document.getElementById("legendWithFieldSet");
  assert_not_equals(legendEle.form, null);
  assert_equals(legendEle.form, document.getElementById("testformWithFieldSet"));
}, "Check if legend.form returns its parent when it's inside a fieldset");
test(function () {
  var legendEle = document.getElementById("testlegend");
  assert_equals(legendEle.form, null);
}, "Check if legend.form return null when legend has no fieldset element as its parent");
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
  "source_name": "html/semantics/forms/the-legend-element/legend-form.html"
}
```
