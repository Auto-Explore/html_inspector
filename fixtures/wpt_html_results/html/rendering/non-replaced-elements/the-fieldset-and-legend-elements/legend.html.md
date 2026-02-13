# html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/legend.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/legend.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>The legend element</title>
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>
<style>
#ref {
  display: block;
  unicode-bidi: isolate;
  padding-left: 2px;
  padding-right: 2px;
  /* TODO: uncomment this when these properties are widely supported
  padding-inline-start: 2px; padding-inline-end: 2px;
  */
}
</style>

<legend id=in-body></legend>
<fieldset>
 <legend id=rendered-legend></legend>
 <legend id=in-fieldset-second-child></legend>
 <div><legend id=in-fieldset-descendant></legend></div>
</fieldset>
<div id=ref></div>

<script>
setup(() => {
  self.legends = [].slice.call(document.querySelectorAll('legend'));
  self.refStyle = getComputedStyle(document.getElementById('ref'));
  self.props = ['display',
                'unicodeBidi',
                'marginTop',
                'marginRight',
                'marginBottom',
                'marginLeft',
                'paddingTop',
                'paddingRight',
                'paddingBottom',
                'paddingLeft',
                'overflow',
                // Extra tests
                'height',
                'box-sizing',
               ];
});
legends.forEach(legend => {
  const testStyle = getComputedStyle(legend);
  props.forEach(prop => {
    test(() => {
      assert_equals(testStyle[prop], refStyle[prop]);
    }, `${legend.id}: ${prop}`);
  });

  // Test width separately since it differs outside fieldset vs. in fieldset vs. rendered legend
  test(() => {
    if (legend.id === 'rendered-legend') {
      assert_equals(testStyle.width, '0px');
    } else {
      assert_not_equals(testStyle.width, '0px');
    }
  }, `${legend.id}: width`);
});
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
  "source_name": "html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/legend.html"
}
```
