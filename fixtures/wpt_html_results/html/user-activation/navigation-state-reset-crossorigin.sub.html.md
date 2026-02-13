# html/user-activation/navigation-state-reset-crossorigin.sub.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/user-activation/navigation-state-reset-crossorigin.sub.html",
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
<body>
  <h1>Post-navigation activation state in child</h1>
  <p>Tests that navigating a cross-origin child frame resets its activation states.</p>
  <ol id="instructions">
    <li>Click inside the yellow area.
  </ol>
  <iframe id="child" width="200" height="50">
  </iframe>
  <script>
    async_test(function(t) {
      var child = document.getElementById("child");
      child.src = "http://{{hosts[alt][]}}:{{ports[http][0]}}/html/user-activation/resources/child-one.html";
      window.addEventListener("message", t.step_func(event => {
          // Test driver can send messages too...
          if (typeof event.data !== "string") return;

          var msg = JSON.parse(event.data);
          if (msg.type == 'child-one-loaded') {
              assert_false(navigator.userActivation.isActive);
              assert_false(navigator.userActivation.hasBeenActive);
              assert_false(msg.isActive);
              assert_false(msg.hasBeenActive);

              delayByFrames(() => test_driver.click(child), 5);
          } else if (msg.type == 'child-one-clicked') {
              assert_true(navigator.userActivation.isActive);
              assert_true(navigator.userActivation.hasBeenActive);
              assert_true(msg.isActive);
              assert_true(msg.hasBeenActive);

              child.src = "http://{{hosts[alt][]}}:{{ports[http][1]}}/html/user-activation/resources/child-two.html";
          } else if (msg.type == 'child-two-loaded') {
              assert_true(navigator.userActivation.isActive);
              assert_true(navigator.userActivation.hasBeenActive);
              assert_false(msg.isActive);
              assert_false(msg.hasBeenActive);

              t.done();
          }
      }));
    }, "Post-navigation state reset.");
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
  "source_name": "html/user-activation/navigation-state-reset-crossorigin.sub.html"
}
```
