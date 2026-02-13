# html/semantics/interactive-elements/commands/legend/first-input-after-legend-manual.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/interactive-elements/commands/legend/first-input-after-legend-manual.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>First input after the legend</title>
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>
<script src=../common/accesskey.js></script>
<p>Press the access key combination for "a". <kbd></kbd></p>
<fieldset>
 <legend accesskey=a>legend</legend>
 <input onfocus="pass()">
</fieldset>
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
  "source_name": "html/semantics/interactive-elements/commands/legend/first-input-after-legend-manual.html"
}
```
