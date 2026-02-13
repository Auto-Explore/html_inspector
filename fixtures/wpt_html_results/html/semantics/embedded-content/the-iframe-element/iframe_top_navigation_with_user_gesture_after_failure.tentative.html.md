# html/semantics/embedded-content/the-iframe-element/iframe_top_navigation_with_user_gesture_after_failure.tentative.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-iframe-element/iframe_top_navigation_with_user_gesture_after_failure.tentative.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>

Check that a cross-site iframe can navigate its top window if the user has
interacted with it, even if it was previously prevented from doing so because of
a missing user activation.

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
      let didReceiveFirstMessage = false;

      window.addEventListener(
        "message",
        t.step_func(function (e) {
          if (!didReceiveFirstMessage) {
            assert_equals(
              e.data,
              "ERROR",
              "On the first attempt, the child should receive an error when trying to navigate"
            );
            assert_equals(
              beforeunloadFired,
              false,
              "On the first attempt, no beforeunload event should have been fired in the parent"
            );
            didReceiveFirstMessage = true;
            return;
          }

          assert_equals(
            e.data,
            "NAVIGATED",
            "On the second attempt, the parent should be navigated"
          );
          assert_equals(
            beforeunloadFired,
            true,
            "On the second attempt, an beforeunload event should have been fired in the parent"
          );
          t.done();
        })
      );

      let parent = open(
        "support/iframe-that-performs-top-navigation-parent-with-user-gesture-after-failure.sub.html"
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
  "source_name": "html/semantics/embedded-content/the-iframe-element/iframe_top_navigation_with_user_gesture_after_failure.tentative.html"
}
```
