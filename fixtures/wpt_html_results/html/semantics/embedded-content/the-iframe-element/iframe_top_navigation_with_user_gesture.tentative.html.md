# html/semantics/embedded-content/the-iframe-element/iframe_top_navigation_with_user_gesture.tentative.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-iframe-element/iframe_top_navigation_with_user_gesture.tentative.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>

Check that a cross-site iframe can navigate its top window if the user has
interacted with it.

<button>Gain user activation</button>

<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-vendor.js"></script>
<script>
  async_test(function (t) {
    // Gain user activation to be able to open a popup. This is needed, as we have
    // enabled popup blocking for this test.
    test_driver.click(document.querySelector("button")).then(() => {
      let beforeunloadFired = false;

      window.addEventListener(
        "message",
        t.step_func_done(function (e) {
          assert_equals(e.data, "NAVIGATED", "The parent should be navigated");
          assert_equals(
            beforeunloadFired,
            true,
            "An beforeunload event should have been fired in the parent"
          );
        })
      );

      let parent = open(
        "support/iframe-that-performs-top-navigation-parent-with-user-gesture.sub.html"
      );
      parent.addEventListener("beforeunload", () => {
        beforeunloadFired = true;
      });
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
  "source_name": "html/semantics/embedded-content/the-iframe-element/iframe_top_navigation_with_user_gesture.tentative.html"
}
```
