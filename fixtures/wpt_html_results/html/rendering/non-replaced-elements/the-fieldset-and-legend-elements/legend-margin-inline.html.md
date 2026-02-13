# html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/legend-margin-inline.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/legend-margin-inline.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>legend and margin (inline direction)</title>
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>
<style>
 fieldset { margin: 0 0 10px 0; padding: 20px; border: 10px solid; width: 500px; }
 legend { height: 10px; width: 200px; padding: 0; }

 #legend-center { margin-left: auto; margin-right: auto; }
 #legend-right { margin-left: auto; }
 #legend-10 { margin-left: 10px; }
</style>
<fieldset>
  <legend id=legend-left>left</legend>
</fieldset>
<fieldset>
  <legend id=legend-center>center</legend>
</fieldset>
<fieldset>
  <legend id=legend-right>right</legend>
</fieldset>
<fieldset>
  <legend id=legend-10>10px</legend>
</fieldset>

<script>
 const legends = document.getElementsByTagName('legend');
 const [legendLeft, legendCenter, legendRight, legend10] = legends;

 const expectedLeft = 8 + 10 + 20;
 const expectedCenter = expectedLeft + (500 / 2) - (200 / 2);
 const expectedRight = expectedLeft + 500 - 200;
 const expected10 = expectedLeft + 10;

 test(() => {
   assert_equals(legendLeft.offsetLeft, expectedLeft);
 }, 'left');

 test(() => {
   assert_equals(legendCenter.offsetLeft, expectedCenter);
 }, 'center');

 test(() => {
   assert_equals(legendRight.offsetLeft, expectedRight);
 }, 'right');

 test(() => {
   assert_equals(legend10.offsetLeft, expected10);
 }, '10px');
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
  "source_name": "html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/legend-margin-inline.html"
}
```
