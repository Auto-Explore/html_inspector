# html/dom/elements/global-attributes/translate-non-html-translation-mode.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/dom/elements/global-attributes/translate-non-html-translation-mode.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset=utf-8>
<title>Non-HTML elements have a translation mode</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script>
  test(() => {
    const svgContainer = document.createElement("svg");
    const foreignObject = document.createElement("foreignObject");
    svgContainer.appendChild(foreignObject);
    const div = document.createElement("div");
    foreignObject.appendChild(div);

    assert_true(div.translate);
  }, 'Non-HTML elements default to translate-enabled');

  test(() => {
    const outerDiv = document.createElement("div");
    outerDiv.translate = true;
    assert_true(outerDiv.translate);

    const svgContainer = document.createElement("svg");
    outerDiv.appendChild(svgContainer);
    const foreignObject = document.createElement("foreignObject");
    svgContainer.appendChild(foreignObject);
    const div = document.createElement("div");
    foreignObject.appendChild(div);

    assert_true(div.translate);
  }, "Non-HTML elements inherit their parent's translation-enabled state");

  test(() => {
    const outerDiv = document.createElement("div");
    outerDiv.translate = false;
    assert_false(outerDiv.translate);

    const svgContainer = document.createElement("svg");
    outerDiv.appendChild(svgContainer);
    const foreignObject = document.createElement("foreignObject");
    svgContainer.appendChild(foreignObject);
    const div = document.createElement("div");
    foreignObject.appendChild(div);

    assert_false(div.translate);
  }, "Non-HTML elements inherit their parent's no-translation state");
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
  "source_name": "html/dom/elements/global-attributes/translate-non-html-translation-mode.html"
}
```
