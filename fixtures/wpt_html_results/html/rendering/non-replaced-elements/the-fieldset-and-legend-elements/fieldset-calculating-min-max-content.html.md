# html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/fieldset-calculating-min-max-content.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/fieldset-calculating-min-max-content.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>fieldset calculating min-/max-content</title>
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>
<style>
fieldset { margin: 0; padding: 0; }
.min-content { width: 0; }
legend { padding: 0; }
fieldset, #ref-max-content, #ref-min-content { float: left; clear: left; border: 1px solid; }
#log { clear: left; }
</style>
<fieldset class=max-content><legend>foo fooo</legend>x x</fieldset>
<fieldset class=max-content><legend>x x</legend>foo fooo</fieldset>
<div id=ref-max-content>foo fooo</div>

<fieldset class=min-content><legend>foo fooo</legend>x x</fieldset>
<fieldset class=min-content><legend>x x</legend>foo fooo</fieldset>
<div id=ref-min-content>fooo</div>

<script>
test(() => {
  const ref = document.querySelector('#ref-max-content');
  for (const e of document.querySelectorAll('.max-content')) {
    assert_equals(e.clientWidth, ref.clientWidth);
  }
}, 'max-content');

test(() => {
  const ref = document.querySelector('#ref-min-content');
  for (const e of document.querySelectorAll('.min-content')) {
    assert_equals(e.clientWidth, ref.clientWidth);
  }
}, 'min-content');
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
  "source_name": "html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/fieldset-calculating-min-max-content.html"
}
```
