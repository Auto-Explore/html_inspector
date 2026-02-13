# html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/legend-align-justify-self.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/legend-align-justify-self.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>legend align to justify-self</title>
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>
<fieldset><legend>x</legend></fieldset>
<fieldset><legend align=left>x</legend></fieldset>
<fieldset><legend align=center>x</legend></fieldset>
<fieldset><legend align=right>x</legend></fieldset>
<fieldset><legend align=lEfT>x</legend></fieldset>
<fieldset><legend align=cEnTeR>x</legend></fieldset>
<fieldset><legend align=rIgHt>x</legend></fieldset>
<!-- invalid values -->
<fieldset><legend align=justify>x</legend></fieldset>
<fieldset><legend align="left ">x</legend></fieldset>
<!-- dir -->
<fieldset><legend dir=ltr>x</legend></fieldset>
<fieldset><legend dir=rtl>x</legend></fieldset>
<fieldset dir=rtl><legend dir=ltr>x</legend></fieldset>
<fieldset dir=rtl><legend dir=rtl>x</legend></fieldset>
<script>
for (const fieldset of document.querySelectorAll('fieldset')) {
  test(() => {
  	const legend = fieldset.firstChild;
  	const align = legend.align.toLowerCase();
  	let expected = 'auto';
    switch (align) {
      case 'left': expected = 'left'; break;
      case 'center': expected = 'center'; break;
      case 'right': expected = 'right'; break;
    }
    assert_equals(getComputedStyle(legend).justifySelf, expected);
  }, `${fieldset.outerHTML}`)
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
  "source_name": "html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/legend-align-justify-self.html"
}
```
