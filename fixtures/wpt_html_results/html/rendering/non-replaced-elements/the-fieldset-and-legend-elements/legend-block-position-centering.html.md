# html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/legend-block-position-centering.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/legend-block-position-centering.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel="help" href="https://html.spec.whatwg.org/C/#the-fieldset-and-legend-elements">
<!-- A test for the following paragraph:
The element is expected to be positioned in the block-flow direction such that
its border box is centered over the border on the block-start side of the
fieldset element.
-->
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<style>
fieldset {
  margin: 0;
  padding: 0;
  border: 100px solid black;
}
legend {
  height: 0px;
  border-color: yellow;
  border-style: solid;
}
</style>
<fieldset>
<legend style="border-width:50px"></legend>
<br>
</fieldset>
<br>

<fieldset>
<legend style="border-width:25px 50px"></legend>
<br>
</fieldset>
<br>

<fieldset>
<legend style="border-width:10px 50px 40px 50px"></legend>
<br>
</fieldset>
<br>

<fieldset>
<legend style="border-width:40px 50px 10px 50px"></legend>
<br>
</fieldset>

<script>
function legendBlockOffset(fieldset) {
  let legend = fieldset.querySelector('legend');
  return legend.getBoundingClientRect().y - fieldset.getBoundingClientRect().y;
}

test(() => {
  let fieldsets = document.querySelectorAll('fieldset');
  assert_equals(legendBlockOffset(fieldsets[0]), 0);
  assert_equals(legendBlockOffset(fieldsets[1]), 25);
  assert_equals(legendBlockOffset(fieldsets[2]), 25);
  assert_equals(legendBlockOffset(fieldsets[3]), 25);
}, 'Legend\'s border-box should be centere on the fieldset border');
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
  "source_name": "html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/legend-block-position-centering.html"
}
```
