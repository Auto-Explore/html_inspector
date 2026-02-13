# html/semantics/interactive-elements/commands/legend/focusable-legend-sibling-manual.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/interactive-elements/commands/legend/focusable-legend-sibling-manual.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>Focusable legend sibling</title>
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>
<script src=../common/accesskey.js></script>
<p>Press the access key combination for "a". <kbd></kbd></p>
<fieldset>
 <legend accesskey=a>first legend</legend>
 <legend tabindex=0 onfocus="fail('sibling legend was focused')">second legend</legend>
</fieldset>
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
  "source_name": "html/semantics/interactive-elements/commands/legend/focusable-legend-sibling-manual.html"
}
```
