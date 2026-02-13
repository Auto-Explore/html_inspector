# html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/legend-align.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/legend-align.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>legend align</title>
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>
<fieldset><legend align=left>x</legend></fieldset>
<fieldset><legend align=center>x</legend></fieldset>
<fieldset><legend align=right>x</legend></fieldset>
<fieldset><legend align=justify>x</legend></fieldset>
<div align=left>
 <fieldset><legend>x</legend></fieldset>
</div>
<div align=center>
 <fieldset><legend>x</legend></fieldset>
</div>
<div align=right>
 <fieldset><legend>x</legend></fieldset>
</div>
<div align=justify>
 <fieldset><legend>x</legend></fieldset>
</div>
<div style="text-align: center">
 <fieldset><legend>x</legend></fieldset>
</div>
<div style="text-align: center" align=center>
 <fieldset><legend>x</legend></fieldset>
</div>
<fieldset><legend style="margin: 0 auto">x</legend></fieldset>
<fieldset><legend style="margin: 0 0 0 auto">x</legend></fieldset>
<fieldset dir=rtl><legend>x</legend></fieldset>
<fieldset dir=rtl><legend style="text-align: left">x</legend></fieldset>
<script>
function test_align(selectorTest, selectorRef) {
  const testElm = document.querySelector(selectorTest);
  const refElm = document.querySelector(selectorRef);
  test(() => {
    assert_equals(testElm.offsetLeft, refElm.offsetLeft, `expected ${selectorRef}`);
  }, selectorTest);
}

for (const val of ['left', 'center', 'right', 'justify']) {
  test_align(`div[align=${val}] legend`, `legend[align=left]`);
}

test_align(`div[style="text-align: center"] legend`, `legend[align=left]`);
test_align(`div[style="text-align: center"][align=center] legend`, `legend[align=left]`);
test_align(`legend[style="margin: 0 auto"]`, `legend[align=center]`);
test_align(`legend[style="margin: 0 0 0 auto"]`, `legend[align=right]`);
test_align(`fieldset[dir=rtl] legend`, `legend[align=right]`);
test_align(`fieldset[dir=rtl] legend[style="text-align: left"]`, `legend[align=right]`);
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
  "source_name": "html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/legend-align.html"
}
```
