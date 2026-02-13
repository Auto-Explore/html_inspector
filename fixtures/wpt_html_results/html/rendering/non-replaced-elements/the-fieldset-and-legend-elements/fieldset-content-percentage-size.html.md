# html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/fieldset-content-percentage-size.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/fieldset-content-percentage-size.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel="help" href="https://crbug.com/1140595">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<div style="height:100px;">
  <fieldset style="margin:0; padding:0; border:none;">
    <div id="inner" style="height:59%;"></div>
  </fieldset>
</div>
<script>
test(() => {
  let innerDiv = document.querySelector('#inner');
  assert_equals(innerDiv.clientHeight, 0);
}, 'A percentage height for an element in an auto-height fieldset should be ignored');
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
  "source_name": "html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/fieldset-content-percentage-size.html"
}
```
