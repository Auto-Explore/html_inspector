# html/semantics/permission-element/display-values.tentative.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/permission-element/display-values.tentative.html",
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
    <title>Permission Element: display style validation</title>
    <link
      rel="help"
      href="https://github.com/WICG/PEPC/blob/main/explainer.md#locking-the-pepc-style"
    />
    <script src="/resources/testharness.js"></script>
    <script src="/resources/testharnessreport.js"></script>
  </head>
  <body>
    <script>
      function createPermissionElementWithStyle(displayValue, type) {
        const element = document.createElement("permission");

        element.setAttribute("type", type);
        element.style.display = displayValue;
        document.body.appendChild(element);
        return element;
      }

      const testCases = [
        ["block", ""],
        ["inline-block", ""],
        ["flex", ""],
        ["inline-flex", ""],
        ["inline", "style_invalid"],
        ["contents", "style_invalid"],
        ["inline-table", "style_invalid"],
        ["list-item", "style_invalid"],
        ["ruby", "style_invalid"],
        ["ruby-text", "style_invalid"],
        ["table", "style_invalid"],
        ["table-caption", "style_invalid"],
        ["table-cell", "style_invalid"],
        ["table-column", "style_invalid"],
        ["table-column-group", "style_invalid"],
        ["table-footer-group", "style_invalid"],
        ["table-header-group", "style_invalid"],
        ["table-row", "style_invalid"],
        ["table-row-group", "style_invalid"],
      ];

      const permissionTypes = ["camera", "microphone", "camera microphone", "geolocation"];

      testCases.forEach((testCase, index) =>
        testCase.push(permissionTypes[index % permissionTypes.length]),
      );
      async_test((t) => {
        let completedTests = 0;

        testCases.forEach(([displayValue, expectedInvalidReason, type]) => {
          const element = createPermissionElementWithStyle(displayValue, type);
          element.onvalidationstatuschange = t.step_func(() => {
            // These two invalid reasons are expected when the permission element was just created.
            if (
              element.invalidReason == "unsuccessful_registration" ||
              element.invalidReason == "intersection_changed"
            ) {
              return;
            }
            assert_equals(
              element.invalidReason,
              expectedInvalidReason,
              `display: ${displayValue} should be ${expectedInvalidReason === "" ? "valid" : "invalid"}`,
            );
            element.remove();
            if (++completedTests === testCases.length) t.done();
          });
        });
      }, "Permission element display style validation");
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
  "source_name": "html/semantics/permission-element/display-values.tentative.html"
}
```
