# html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/legend-align-text-align.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/legend-align-text-align.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>legend align does not map to text-align</title>
<!-- See discussion in https://bugzilla.mozilla.org/show_bug.cgi?id=1488228 -->
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>
<style>
 legend { width: 13em }
</style>
<fieldset><legend>foo bar abcdefghijklmnopqrstuvwxyz</legend></fieldset>
<fieldset><legend align=left>foo bar abcdefghijklmnopqrstuvwxyz</legend></fieldset>
<fieldset><legend align=center>foo bar abcdefghijklmnopqrstuvwxyz</legend></fieldset>
<fieldset><legend align=right>foo bar abcdefghijklmnopqrstuvwxyz</legend></fieldset>
<fieldset><legend align=justify>foo bar abcdefghijklmnopqrstuvwxyz</legend></fieldset>
<script>
function test_align(selectorTest, expectedAlign) {
  const testElm = document.querySelector(selectorTest);
  test(() => {
    assert_equals(getComputedStyle(testElm).textAlign, expectedAlign);
  }, selectorTest);
}

test_align('legend', 'start');

for (const val of ['left', 'center', 'right', 'justify']) {
  test_align(`legend[align=${val}]`, 'start');
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
  "source_name": "html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/legend-align-text-align.html"
}
```
