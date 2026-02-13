# html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/crashtests/fieldset-middleclick.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/crashtests/fieldset-middleclick.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE HTML>
<html class="test-wait">
<head>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-actions.js"></script>
<script src="/resources/testdriver-vendor.js"></script>
<style>
#f {
  overflow-y: scroll;
  width: 100px;
  height: 100px;
}
</style>
</head>
<body>
<fieldset id="f">
  <p>test</p>
  <p>test</p>
  <p>test</p>
  <p>test</p>
  <p>test</p>
  <p>test</p>
</fieldset>
<script>
onload = async () => {
  const actions = new test_driver.Actions();
  const button = {button: actions.ButtonType.MIDDLE};
  await actions.
      pointerMove(50, 50).
      pointerDown(button).
      pointerUp(button).
      send();
  document.documentElement.className = "";
};
</script>
</body>
</html>
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
  "source_name": "html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/crashtests/fieldset-middleclick.html"
}
```
