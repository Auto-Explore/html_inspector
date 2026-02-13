# html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/legend-display-none.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/legend-display-none.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>legend display: none</title>
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>
<style>
 legend { display: none; }
</style>
<fieldset>
 <legend>Foo</legend>
</fieldset>
<script>
 test(() => {
   const display = getComputedStyle(document.querySelector('legend')).display;
   assert_equals(display, 'none');
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
  "source_name": "html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/legend-display-none.html"
}
```
