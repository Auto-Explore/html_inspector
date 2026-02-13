# html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/legend-inline-position-with-fieldset-padding.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/legend-inline-position-with-fieldset-padding.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel="help" href="https://html.spec.whatwg.org/C/#the-fieldset-and-legend-elements">
<!-- A test for the following paragraphs:
The element's box is expected to be constrained in the inline direction by
the inline content size of the fieldset as if it had used its computed
inline padding.
Example:
For example, if the fieldset has a specified padding of 50px, then the
rendered legend will be positioned 50px in from the fieldset's border. The
padding will further apply to the anonymous fieldset content box instead
of the fieldset element itself.
-->
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<style>
fieldset {
  width: 400px;
  margin: 0;
  padding: 0 50px;
  border: 2px solid black;
}
legend {
  width: 100%;
  padding: 0;
  background-color: yellow;
}
.content {
  background-color: lime;
}
</style>
<fieldset>
<legend>Legend</legend>
</fieldset>

<script>
test(() => {
  let fieldset = document.querySelector('fieldset');
  let legend = document.querySelector('legend');
  assert_equals(legend.offsetLeft - fieldset.offsetLeft, 52);
  assert_equals(legend.offsetWidth, 400);
}, 'Test legend\'s inline-size in a fieldset with inline paddings');
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
  "source_name": "html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/legend-inline-position-with-fieldset-padding.html"
}
```
