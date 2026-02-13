# html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/fieldset-overflow-cssomview.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/fieldset-overflow-cssomview.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<style>
fieldset {
  height: 200px;
  overflow: scroll;
  padding: 0;
}

.content {
  height: 400px;
}
</style>

<fieldset>
  <legend>Legend</legend>
  <div class="content"></div>
</fieldset>

<script>
test(() => {
  const fieldset = document.querySelector('fieldset');
  assert_equals(getComputedStyle(fieldset)['overflow-x'], 'scroll');
  assert_equals(getComputedStyle(fieldset)['overflow-y'], 'scroll');
  assert_equals(fieldset.scrollHeight, 400);
  fieldset.scrollTop = 500;
  assert_greater_than_equal(fieldset.scrollTop, 200);
}, 'Test cssom-view API for FIELDSET');
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
  "source_name": "html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/fieldset-overflow-cssomview.html"
}
```
