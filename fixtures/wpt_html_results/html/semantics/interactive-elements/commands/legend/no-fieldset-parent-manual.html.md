# html/semantics/interactive-elements/commands/legend/no-fieldset-parent-manual.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/interactive-elements/commands/legend/no-fieldset-parent-manual.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>No fieldset parent</title>
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>
<script src=../common/accesskey.js></script>
<p>Press the access key combination for "a". <kbd></kbd></p>
<legend accesskey=a>
 legend
 <input onfocus="fail('input in legend was focused')">
</legend>
<input onfocus="fail('input after legend was focused')">
<script>
 onkeyup = e => {
   if (e.keyCode === 65) {
     pass();
   }
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
  "source_name": "html/semantics/interactive-elements/commands/legend/no-fieldset-parent-manual.html"
}
```
