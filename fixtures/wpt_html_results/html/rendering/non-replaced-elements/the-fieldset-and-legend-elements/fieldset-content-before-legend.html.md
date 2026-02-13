# html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/fieldset-content-before-legend.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/fieldset-content-before-legend.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>fieldset content before legend</title>
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>
<style>
fieldset {
  /* Paddings might have fractional values by default, and they can cause
     rounding differences between the test element and the reference element. */
  padding: 0;
}
</style>

<fieldset id=test>
  X
  <legend>legend</legend>
  Y
</fieldset>
<fieldset id=ref>
  <legend>legend</legend>
  X Y
</fieldset>

<fieldset id="test2">
  P<span id="hidden" style="display:none;">AS</span><legend>legend</legend>S
</fieldset>

<script>
  test(() => {
    const testElm = document.getElementById('test');
    const refElm = document.getElementById('ref');
    assert_equals(testElm.clientHeight, refElm.clientHeight);
  });

  test(() => {
    const testElm = document.getElementById('test2');
    testElm.clientHeight;
    const span = document.getElementById('hidden');
    span.style.display = 'inline';
    testElm.clientHeight;
  }, 'Showing a node just before the rendered legend should not crash');
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
  "source_name": "html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/fieldset-content-before-legend.html"
}
```
