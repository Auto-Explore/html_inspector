# html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/legend-display-ruby.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/legend-display-ruby.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>rendered legend and CSS display (ruby)</title>
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>
<style>
legend { width:initial; }
</style>
<fieldset><legend id="ref">x</legend></fieldset>
<fieldset><legend id="test">x</legend></fieldset>
<script>
  const refElm = document.querySelector('#ref');
  const refStyle = getComputedStyle(refElm);
  const testElm = document.querySelector('#test');
  // Note that we're not testing "display:run-in" here; it's mentioned in
  // several CSS specs, but no browser engines appear likely to support it.
  const values = ['block ruby', 'ruby', 'ruby-base', 'ruby-text', 'ruby-base-container', 'ruby-text-container'];
  const extraStyle = ['', 'overflow:hidden', 'columns:1', 'overflow:hidden;columns:1'];

  for (const style of extraStyle) {
    for (const val of values) {
      test(() => {
        testElm.style.removeProperty('display');
        testElm.style = style;
        testElm.style.display = val;
        const computed = getComputedStyle(testElm);
        // Note that computed value is different from the used value.
        // E.g., if ruby is not supported, the following assertion will
        // fail as the computed value of display will be block.
        // If ruby is supported, computed.display will return "ruby",
        // but the used value is supposed to be "block ruby".
        let expected = val;
        assert_equals(computed.display, expected, `display: ${val} is not supported`);
        assert_equals(computed.width, refStyle.width, 'width');
        assert_equals(testElm.offsetLeft, refElm.offsetLeft, 'offsetLeft');
      }, `rendered legend with display: ${val}` + (style == '' ? '' : "; " + style));
    }
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
  "source_name": "html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/legend-display-ruby.html"
}
```
