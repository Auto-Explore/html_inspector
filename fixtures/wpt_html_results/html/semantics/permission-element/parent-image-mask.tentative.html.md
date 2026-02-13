# html/semantics/permission-element/parent-image-mask.tentative.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/permission-element/parent-image-mask.tentative.html",
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
    <title>Permission Element: invalid if parent has mask-image</title>
    <link
      rel="help"
      href="https://github.com/WICG/PEPC/blob/main/explainer.md#locking-the-pepc-style"
    />
    <script src="/resources/testharness.js"></script>
    <script src="/resources/testharnessreport.js"></script>
  </head>
  <body>
    <script>
      async_test((t) => {
        const parent = document.createElement("div");
        const element = document.createElement("permission");
        element.setAttribute("type", "camera");
        parent.appendChild(element);

        parent.style.maskImage = "url(mask.png)";

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
            "style_invalid",
            `Parent with mask-image should be invalid`,
          );
          t.done();
        });

        document.body.appendChild(parent);
      }, "Permission element is invalid if it has a parent with mask-image.");
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
  "source_name": "html/semantics/permission-element/parent-image-mask.tentative.html"
}
```
