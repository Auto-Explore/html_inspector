# html/semantics/permission-element/granted-selector.html

Counts:
- errors: 0
- warnings: 4
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/permission-element/granted-selector.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset=utf-8>
<link rel="help" href="https://github.com/WICG/PEPC/blob/main/explainer.md#locking-the-pepc-style">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-actions.js"></script>
<script src="/resources/testdriver-vendor.js"></script>
<body>

<permission id="permission_element" type="camera"></permission>

<script>
  promise_test(async() => {

    // Set the initial camera state to denied.
    await test_driver.set_permission({name: "camera"}, "denied");
    await navigator.permissions.query({name: "camera"});

    assert_false(permission_element.matches(":granted"));

    // Set the camera state to allowed.
    await test_driver.set_permission({name: "camera"}, "granted");
    await navigator.permissions.query({name: "camera"});

    // The granted selector should now be applied.
    assert_true(permission_element.matches(":granted"));

    // Set the camera state to denied.
    await test_driver.set_permission({name: "camera"}, "denied");
    await navigator.permissions.query({name: "camera"});

    // The granted selector should now be removed.
    assert_false(permission_element.matches(":granted"));

  }, "Permission element should not have the granted selector when the \
      permission is not granted.")
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “permission” not allowed as child of “body” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 463,
        "byte_start": 413,
        "col": 1,
        "line": 11
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “permission” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 463,
        "byte_start": 413,
        "col": 1,
        "line": 11
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
  "source_name": "html/semantics/permission-element/granted-selector.html"
}
```
