# html/interaction/focus/processing-model/legend.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/interaction/focus/processing-model/legend.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>legend</title>
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>
<script>
 const t = async_test();
</script>
<fieldset>
 <legend onfocus="t.unreached_func('legend was focused')()">
  legend
  <input onfocus="t.unreached_func('input in legend was focused')();">
 </legend>
 <input onfocus="t.unreached_func('input after legend was focused')();">
</fieldset>
<script>
  document.querySelector('legend').focus();
  t.step_timeout(() => {
    t.done();
  }, 500);
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
  "source_name": "html/interaction/focus/processing-model/legend.html"
}
```
