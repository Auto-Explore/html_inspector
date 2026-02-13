# html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/min-inline-size.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/min-inline-size.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>fieldset min-inline-size</title>
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>
<style>
 fieldset { width: 0; height: 0 }
 fieldset > div { width: 100px; height: 100px }
 #vertical-lr { writing-mode: vertical-lr }
 #vertical-rl { writing-mode: vertical-rl }
 .override { min-inline-size: 5px }
</style>
<fieldset id=horizontal-tb><div></div></fieldset>
<fieldset id=vertical-lr><div></div></fieldset>
<fieldset id=vertical-rl><div></div></fieldset>
<script>
  for (const className of ['', 'override']) {
    const expected = className === '' ? '100px' : '5px';
    test(() => {
      const fieldset = document.getElementById('horizontal-tb');
      fieldset.className = className;
      assert_equals(getComputedStyle(fieldset).width, expected, 'width');
      assert_equals(getComputedStyle(fieldset).height, '0px', 'height');
    }, `horizontal-tb ${className}`);

    test(() => {
      const fieldset = document.getElementById('vertical-lr');
      fieldset.className = className;
      assert_equals(getComputedStyle(fieldset).width, '0px', 'width');
      assert_equals(getComputedStyle(fieldset).height, expected, 'height');
    }, `vertical-lr ${className}`);

    test(() => {
      const fieldset = document.getElementById('vertical-rl');
      fieldset.className = className;
      assert_equals(getComputedStyle(fieldset).width, '0px', 'width');
      assert_equals(getComputedStyle(fieldset).height, expected, 'height');
    }, `vertical-rl ${className}`);
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
  "source_name": "html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/min-inline-size.html"
}
```
