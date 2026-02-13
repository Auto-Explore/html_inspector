# html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/fieldset-block-size.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/fieldset-block-size.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel="help" href="https://html.spec.whatwg.org/C/#the-fieldset-and-legend-elements">
<!-- A test for the following paragraph:
For the purpose of calculating the used 'block-size', if the computed
'block-size' is not 'auto', the space allocated for the rendered legend's
margin box that spills out past the border, if any, is expected to be
subtracted from the 'block-size'. If the content box's block-size would be
negative, then let the content box's block-size be zero instead.
-->
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<style>
fieldset {
  margin: 0;
  padding: 0;
  border: 2px solid black;
}
legend {
  height: 102px;
  background-color: yellow;
}
.content {
  background-color: lime;
}
</style>
<fieldset style="block-size: 200px;">
<legend>Legend</legend>
<div class="content" style="height:100%"></div>
</fieldset>

<fieldset style="block-size: 40px;">
<legend>Legend</legend>
<div class="content" style="height:100%"></div>
</fieldset>

<script>
test(() => {
  let cs = document.querySelectorAll('.content');
  assert_equals(cs[0].offsetHeight, Math.max(202 - 102, 0));
  assert_equals(cs[1].offsetHeight, Math.max(42 - 102, 0));
}, 'Test content\'s block-size');
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
  "source_name": "html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/fieldset-block-size.html"
}
```
