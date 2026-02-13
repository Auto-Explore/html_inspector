# html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/fieldset-block-formatting-context.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/fieldset-block-formatting-context.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>The fieldset element: block formatting context</title>
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>
<style>
* {
  margin: 0;
  padding: 0;
}
fieldset { border: none; }
.float {
  float: left;
  width: 50px;
  height: 50px;
  background-color: orange;
}
</style>

<div class=float></div>
<fieldset><div class=float></div></fieldset>

<script>
test(() => {
  const fieldset = document.querySelector('fieldset');
  assert_equals(fieldset.offsetTop, 0, 'fieldset.offsetTop');
  assert_equals(fieldset.offsetLeft, 50, 'fieldset.offsetLeft');
  assert_equals(fieldset.clientHeight, 50, 'fieldset.clientHeight');
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
  "source_name": "html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/fieldset-block-formatting-context.html"
}
```
