# html/user-activation/message-event-activation-api-iframe-cross-origin.sub.tentative.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/user-activation/message-event-activation-api-iframe-cross-origin.sub.tentative.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<!--
   Tentative due to:
   https://github.com/whatwg/html/issues/1983
-->
<html>
<head>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-vendor.js"></script>
</head>
<body>
  <h1>Clicking in iframe has activation state in child via MessageEvent</h1>
  <ol id="instructions">
    <li>Click inside the red area.
  </ol>
  <iframe id="child" width="200" height="200">
  </iframe>
  <script>
    async_test(function(t) {
      var child = document.getElementById("child");
      child.src = "http://{{hosts[alt][]}}:{{ports[http][0]}}/html/user-activation/resources/child-message-event-api.html";
      assert_false(navigator.userActivation.isActive);
      assert_false(navigator.userActivation.hasBeenActive);
      window.addEventListener("message", t.step_func(event => {
        if (event.data == 'child-loaded') {
          // values have false after load
          assert_true(event.userActivation != null);
          assert_false(event.userActivation.isActive);
          assert_false(event.userActivation.hasBeenActive);
          test_driver.click(child);
        } else if (event.data == 'child-clicked') {
          // values have activation state on click
          assert_true(navigator.userActivation.hasBeenActive);
          assert_true(event.userActivation != null);
          assert_true(event.userActivation.isActive);
          assert_true(event.userActivation.hasBeenActive);
          child.contentWindow.postMessage('report', "*");
        } else if (event.data == 'child-report') {
          assert_false(navigator.userActivation.isActive);
          assert_true(navigator.userActivation.hasBeenActive);
          assert_true(event.userActivation != null);
          assert_false(event.userActivation.isActive);
          assert_true(event.userActivation.hasBeenActive);
          child.contentWindow.postMessage('report-no-activation', "*");
        } else if (event.data == 'child-report-no-activation') {
          assert_equals(event.userActivation, null);
          t.done();
        }
      }));
    }, "Message propagates values on post");
  </script>
</body>
</html>
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
  "source_name": "html/user-activation/message-event-activation-api-iframe-cross-origin.sub.tentative.html"
}
```
