# html/semantics/permission-element/no-focus.tentative.html

Counts:
- errors: 0
- warnings: 6
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/permission-element/no-focus.tentative.html",
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
<!-- The permission element should not be focusable by script.
-->
<permission tabindex="0" id="valid_permission_element" type="geolocation"></permission>

<span tabindex="0" id="focusable_span">This is some text</span>

<!-- style needed to allow the invalid element to have a width -->
<permission style="width: auto; padding-left: 10px" tabindex="0" id="invalid_permission_element" type="invalid"></permission>

<script>
  promise_test(async() => {
    invalid_permission_element.focus();
    assert_equals(document.activeElement, invalid_permission_element,
      "Invalid permission element should be focusable");

    focusable_span.focus();
    valid_permission_element.focus();
    assert_equals(document.activeElement, focusable_span,
      "Permission element should not be focused.");

    focusable_span.focus();
    await test_driver.bless('Focus with user activation', () => {
      valid_permission_element.focus();
    });
    assert_equals(document.activeElement, valid_permission_element,
      "Focus is allowed with user activation");

    focusable_span.focus();
    actions = new test_driver.Actions()
       .pointerMove(1, 1, {origin: valid_permission_element})
       .pointerDown()
       .addTick();
    await actions.send();
    assert_equals(document.activeElement, valid_permission_element,
      "Users can focus the element");

    focusable_span.focus();
    assert_equals(document.activeElement, focusable_span,
        "Other element should be focused");
}, "Permission element is not focusable by script without user activation");
</script>
</body>
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
        "byte_end": 553,
        "byte_start": 479,
        "col": 1,
        "line": 12
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “permission” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 553,
        "byte_start": 479,
        "col": 1,
        "line": 12
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “permission” not allowed as child of “body” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 812,
        "byte_start": 700,
        "col": 1,
        "line": 17
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “permission” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 812,
        "byte_start": 700,
        "col": 1,
        "line": 17
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
  "source_name": "html/semantics/permission-element/no-focus.tentative.html"
}
```
