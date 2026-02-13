# html/user-activation/detached-iframe.html

Counts:
- errors: 3
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/user-activation/detached-iframe.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
  <head>
    <script src="/resources/testharness.js"></script>
    <script src="/resources/testharnessreport.js"></script>
    <script src="/resources/testdriver.js"></script>
    <script src="/resources/testdriver-vendor.js"></script>
    <script src="resources/utils.js"></script>
  </head>
  <body></body>
  <script>
    async function attachIframe() {
      const iframe = document.createElement("iframe");
      iframe.src = "about:blank";
      await new Promise((r) => {
        iframe.addEventListener("load", r, { once: true });
        document.body.append(iframe);
      });
      return iframe;
    }

    promise_test(async () => {
      const iframe = await attachIframe();
      const { userActivation } = iframe.contentWindow.navigator;

      assert_false(
        userActivation.isActive,
        "No transient activation before click"
      );
      assert_false(
        userActivation.hasBeenActive,
        "No sticky activation before click"
      );

      // Confirm we have activation
      await test_driver.bless("click", null, iframe.contentWindow);
      assert_true(userActivation.isActive, "is active after click");
      assert_true(userActivation.hasBeenActive, "has been active");

      // Remove the context
      iframe.remove();
      assert_equals(iframe.contentWindow, null, "No more global");
      assert_true(userActivation.isActive, "isActive");
      assert_true(userActivation.hasBeenActive, "hasBeenActive");
    }, "navigator.userActivation retains state even if global is removed");
  </script>
</html>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.parser.stray_start_tag",
      "message": "Stray start tag “script”.",
      "severity": "Error",
      "span": {
        "byte_end": 342,
        "byte_start": 334,
        "col": 3,
        "line": 11
      }
    },
    {
      "category": "Html",
      "code": "html.parser.nonspace_after_body",
      "message": "Non-space character after body.",
      "severity": "Error",
      "span": {
        "byte_end": 1558,
        "byte_start": 342,
        "col": 11,
        "line": 11
      }
    },
    {
      "category": "Html",
      "code": "html.parser.end_tag_after_body_closed",
      "message": "Saw an end tag after “body” had been closed.",
      "severity": "Error",
      "span": {
        "byte_end": 1567,
        "byte_start": 1558,
        "col": 3,
        "line": 46
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
  "source_name": "html/user-activation/detached-iframe.html"
}
```
