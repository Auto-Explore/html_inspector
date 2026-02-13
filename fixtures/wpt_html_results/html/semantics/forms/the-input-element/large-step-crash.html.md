# html/semantics/forms/the-input-element/large-step-crash.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-input-element/large-step-crash.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel=help href="https://bugs.chromium.org/p/chromium/issues/detail?id=1310229">
<script>
const input = document.createElement('input');
input.type = 'range';
input.max =  "06146014076123948948236985915694585937453938739248525313667193356954648912174625325457686181245605159230507050382951965923880139416566171456307667108838599671206701390275757535304375074544995161254818024615";
input.step = "55244276720723476767813103100759083382064508394993167470137";
input.stepUp();
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.head.title.missing",
      "message": "Element “head” is missing a required instance of child element “title”.",
      "severity": "Warning",
      "span": null
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/semantics/forms/the-input-element/large-step-crash.html"
}
```
