# html/semantics/interactive-elements/commands/legend/label-sibling-manual.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/interactive-elements/commands/legend/label-sibling-manual.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>Label sibling</title>
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>
<script src=../common/accesskey.js></script>
<p>Press the access key combination for "a". <kbd></kbd></p>
<input id=x onfocus="fail('input associated with the label was focused')">
<fieldset>
 <legend accesskey=a>legend</legend>
 <label for=x onclick="fail('label received a click event')">label</label>
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
  "source_name": "html/semantics/interactive-elements/commands/legend/label-sibling-manual.html"
}
```
