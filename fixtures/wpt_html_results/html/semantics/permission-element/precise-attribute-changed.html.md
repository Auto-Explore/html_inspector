# html/semantics/permission-element/precise-attribute-changed.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/permission-element/precise-attribute-changed.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<html>
  <head>
    <meta charset="utf-8" />
    <title>Permission Element: invalid if precise attribute is changed</title>
    <link rel="help" href="https://github.com/WICG/PEPC/blob/main/explainer.md" />
    <script src="/resources/testharness.js"></script>
    <script src="/resources/testharnessreport.js"></script>
  </head>
  <body>
    <script>
      promise_test(async (t) => {
        let is_element_valid = false;
        const element = document.createElement("permission");
        element.setAttribute("type", "geolocation");
        element.onvalidationstatuschange = t.step_func(() => {
          if (element.invalidReason == "") {
            is_element_valid = true;
          }
          if ( element.invalidReason == "attribute_changed" ||
              element.invalidReason == "intersection_changed") {
            is_element_valid = false;
          }
        });
        document.body.appendChild(element);

        await t.step_wait(() => is_element_valid === true, "Wait for the element to be ready.");

        // setting the preciselocation attribute should temporarily disable the element.
        element.setAttribute("preciselocation", "");
        await t.step_wait(
          () => is_element_valid === false,
          "Element should become invalid after preciselocation is set.",
        );
        await t.step_wait(
          () => is_element_valid === true,
          "Element should automatically become valid again after 500ms after the attribute is set.",
        );

        // uusetting the preciselocation attribute should temporarily disable the element.
        element.removeAttribute("preciselocation");
        await t.step_wait(
          () => is_element_valid === false,
          "Element should become invalid after preciselocation is unset.",
        );
        await t.step_wait(
          () => is_element_valid === true,
          "Element should automatically become valid again after 500ms after the attribute is unset.",
        );
      }, "Permission element is invalid temporarily if the precise attribute is changed.");
    </script>
  </body>
</html>
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
  "source_name": "html/semantics/permission-element/precise-attribute-changed.html"
}
```
