# html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/fieldset-multicol.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/fieldset-multicol.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>fieldset multicol</title>
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>
<style>
 #test { margin: 0; padding: 0; border: none }
 #test, #ref { columns: 5 }
 p { margin: 0 }
</style>
<fieldset id=test>
  <p>1
  <p>2
  <p>3
  <p>4
  <p>5
</fieldset>
<div id=ref>
  <p>1
  <p>2
  <p>3
  <p>4
  <p>5
</div>
<script>
  test(() => {
    assert_equals(getComputedStyle(document.getElementById('test')).height,
                  getComputedStyle(document.getElementById('ref')).height);
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
  "source_name": "html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/fieldset-multicol.html"
}
```
