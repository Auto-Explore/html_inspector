# html/semantics/interactive-elements/commands/legend/input-outside-fieldset-manual.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/interactive-elements/commands/legend/input-outside-fieldset-manual.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>Input outside fieldset</title>
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>
<script src=../common/accesskey.js></script>
<p>Press the access key combination for "a". <kbd></kbd></p>
<fieldset>
 <legend accesskey=a>legend</legend>
</fieldset>
<input onfocus="fail('input outside fieldset was focused')">
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
  "source_name": "html/semantics/interactive-elements/commands/legend/input-outside-fieldset-manual.html"
}
```
