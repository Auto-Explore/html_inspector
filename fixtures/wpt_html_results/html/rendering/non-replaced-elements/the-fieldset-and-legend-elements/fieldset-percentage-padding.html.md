# html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/fieldset-percentage-padding.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/fieldset-percentage-padding.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>fieldset percentage padding</title>
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>
<style>
 body { margin: 0; }
 .outer { width: 500px; position: relative; }
 fieldset { width: 100px; padding: 10%; margin: 0; border: none; }
 .overflow { overflow: auto; }
</style>
<div class=outer>
 <fieldset>
  <div id=no-overflow>x</div>
 </fieldset>
</div>
<div class=outer>
 <fieldset class=overflow>
  <div id=with-overflow>x</div>
 </fieldset>
</div>
<script>
const noOverflow = document.getElementById('no-overflow');
const withOverflow = document.getElementById('with-overflow');
for (const div of [noOverflow, withOverflow]) {
  test(() => {
    assert_equals(div.offsetLeft, 50, "offsetLeft");
    assert_equals(div.clientWidth, 100, "clientWidth");
    assert_equals(div.offsetTop, 50, "offsetTop");
  }, div.id);
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
  "source_name": "html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/fieldset-percentage-padding.html"
}
```
