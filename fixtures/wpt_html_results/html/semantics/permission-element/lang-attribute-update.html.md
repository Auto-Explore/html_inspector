# html/semantics/permission-element/lang-attribute-update.html

Counts:
- errors: 0
- warnings: 4
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/permission-element/lang-attribute-update.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
<meta charset=utf-8>
<link rel="help" href="https://github.com/WICG/PEPC/blob/main/explainer.md#locking-the-pepc-style">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-actions.js"></script>
<script src="/resources/testdriver-vendor.js"></script>
<body>
  <div id="el1" lang="en">
    <div id="el2" lang="en">
      <div>
        <permission id="permission-element" type="geolocation" style="width:fit-content"></permission>
      </div>
    </div>
  </div>

<script>
  // Since the `lang` attribute is inherited, but the actual inherited value
  // isn't available via IDL, there's no direct way to check that it gets
  // invalidated and updated when changes are made. As such, this test looks
  // for side-effects of changing the language, such as changing the rendered
  // size of the element.
  promise_test(async() => {
    var permission_element = document.getElementById("permission-element");
    const initial_width = permission_element.offsetWidth;
    let widths = new Set();
    widths.add(initial_width);
    const outer_lang_div = document.getElementById("el1");
    const inner_lang_div = document.getElementById("el2");

    // Changing the lang of the outer div should not have any effect as it is
    // shadowed by the inner div.
    outer_lang_div.lang = "de";
    assert_equals(permission_element.offsetWidth, initial_width);

    // The width of the permission element should change due to the changed
    // language of the inner element. Try a couple languages to make sure one
    // of them has a different size.
    ['de','hu','fr-AG','es'].forEach(lang => {
      inner_lang_div.lang = lang;
      widths.add(permission_element.offsetWidth);
    });
    assert_true(widths.size > 1);

  }, "Permission element should dynamically change text when the lang \
      attribute changes")
</script>
</body>
</html>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “permission” not allowed as child of “div” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 576,
        "byte_start": 495,
        "col": 9,
        "line": 14
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “permission” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 576,
        "byte_start": 495,
        "col": 9,
        "line": 14
      }
    },
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
  "source_name": "html/semantics/permission-element/lang-attribute-update.html"
}
```
