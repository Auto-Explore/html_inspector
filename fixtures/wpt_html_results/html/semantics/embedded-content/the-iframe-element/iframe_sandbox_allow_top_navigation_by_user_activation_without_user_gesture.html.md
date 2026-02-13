# html/semantics/embedded-content/the-iframe-element/iframe_sandbox_allow_top_navigation_by_user_activation_without_user_gesture.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-iframe-element/iframe_sandbox_allow_top_navigation_by_user_activation_without_user_gesture.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>

<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-vendor.js"></script>
<script>
  async_test(function (t) {
    let beforeunloadFired = false;

    window.addEventListener(
      "message",
      t.step_func_done(function (e) {
        assert_equals(
          e.data,
          "ERROR",
          "The child should receive an error when trying to navigate"
        );
        assert_equals(
          beforeunloadFired,
          false,
          "No beforeunload event should have been fired in the parent"
        );
      })
    );

    let parent = open(
      "support/iframe-that-performs-top-navigation-parent-with-sandbox.sub.html"
    );
    parent.addEventListener("beforeunload", () => {
      beforeunloadFired = true;
    });
  });
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
  "source_name": "html/semantics/embedded-content/the-iframe-element/iframe_sandbox_allow_top_navigation_by_user_activation_without_user_gesture.html"
}
```
