# html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/fieldset-border-radius-hittest.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/fieldset-border-radius-hittest.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>fieldset, border-radius and hit testing</title>
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>
<style>
fieldset { width: 80px; height: 80px; border-radius: 100px; border: 10px solid; background: lime; }
</style>
<fieldset>
</fieldset>
<script>
test(() => {
  assert_equals(document.elementFromPoint(20, 20), document.body);
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
  "source_name": "html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/fieldset-border-radius-hittest.html"
}
```
