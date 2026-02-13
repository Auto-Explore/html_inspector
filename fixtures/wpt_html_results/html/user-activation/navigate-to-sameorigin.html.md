# html/user-activation/navigate-to-sameorigin.html

Counts:
- errors: 3
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/user-activation/navigate-to-sameorigin.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>User activation propagation across a same-origin navigation</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-vendor.js"></script>
<script src="/resources/testdriver-actions.js"></script>
<script src="resources/utils.js"></script>

<body>
  <p>Placeholder</p>
</body>

<script>
  'use strict';

  let w;
  document.body.onclick = () => {
    w = window.open("resources/opened-window.html");
  };

  promise_test(async test => {
    let window_opened_msg_promise = receiveMessage("window-opened");

    await test_driver.click(document.body);
    assert_true(!!w, "A window is opened");

    {
      let window_opened_data = await window_opened_msg_promise;
      assert_false(window_opened_data.isActive,
          "Transient activation after window opened");
      assert_false(window_opened_data.hasBeenActive,
          "Sticky activation after window opened");
    }

    let link_clicked_msg_promise = receiveMessage("link-clicked");
    let window_navigated_msg_promise = receiveMessage("window-navigated");

    const link = w.document.getElementById("link");
    await test_driver.click(link);

    {
      let link_clicked_data = await link_clicked_msg_promise;
      assert_true(link_clicked_data.isActive,
          "Transient activation after link clicked");
      assert_true(link_clicked_data.hasBeenActive,
          "Sticky activation after link clicked");
    }

    {
      let window_navigated_data = await window_navigated_msg_promise;
      assert_false(window_navigated_data.isActive,
          "Transient activation after window navigated");
      assert_true(window_navigated_data.hasBeenActive,
          "Sticky activation after window navigated");
    }
  }, "User activation propagation across a same-origin navigation");
</script>
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
        "byte_end": 448,
        "byte_start": 440,
        "col": 1,
        "line": 14
      }
    },
    {
      "category": "Html",
      "code": "html.parser.nonspace_after_body",
      "message": "Non-space character after body.",
      "severity": "Error",
      "span": {
        "byte_end": 1921,
        "byte_start": 448,
        "col": 9,
        "line": 14
      }
    },
    {
      "category": "Html",
      "code": "html.parser.end_tag_after_body_closed",
      "message": "Saw an end tag after “body” had been closed.",
      "severity": "Error",
      "span": {
        "byte_end": 1930,
        "byte_start": 1921,
        "col": 1,
        "line": 58
      }
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/user-activation/navigate-to-sameorigin.html"
}
```
