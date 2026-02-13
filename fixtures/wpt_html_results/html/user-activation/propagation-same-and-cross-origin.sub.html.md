# html/user-activation/propagation-same-and-cross-origin.sub.html

Counts:
- errors: 3
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/user-activation/propagation-same-and-cross-origin.sub.html",
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
    <h1>Check that cross origin iframes don't get activated</h1>
    <p>
      Tests that activating a same-origin navigable doesn't activate a cross
      origin navigable.
    </p>
    <ol id="instructions">
      <li>Click inside the yellow area.</li>
    </ol>
    <h1>Same origin</h1>
    <iframe id="so-child" width="200" height="50"></iframe>
    <h1>Cross origin</h1>
    <iframe id="xo-child" width="200" height="50"></iframe>
  </body>
  <script>
    const soChild = document.getElementById("so-child");
    const xoChild = document.getElementById("xo-child");

    function requestXOReport() {
      xoChild.contentWindow.postMessage(
        JSON.stringify({ type: "report" }),
        "*"
      );
      return receiveMessage(`child-two-report`);
    }

    promise_setup(() => {
      soChild.src = "./resources/child-one.html";
      xoChild.src =
        "http://{{hosts[alt][]}}:{{ports[http][0]}}/html/user-activation/resources/child-two.html";
      return Promise.all([
        receiveMessage("child-one-loaded"),
        receiveMessage("child-two-loaded"),
      ]);
    });

    promise_test(async (t) => {
      const unclickedCrossOrigin = await requestXOReport();
      const soActivation = soChild.contentWindow.navigator.userActivation;
      assert_false(
        navigator.userActivation.isActive,
        "top-frame navigator.userActivation.isActive must be false"
      );
      assert_false(
        navigator.userActivation.hasBeenActive,
        "top-frame navigator.userActivation.hasBeenActive must be false"
      );

      assert_false(soActivation.isActive, "child-one isActive must be false");
      assert_false(
        soActivation.hasBeenActive,
        "child-one hasBeenActive must be false"
      );
      assert_false(
        unclickedCrossOrigin.isActive,
        "child-two isActive must be false"
      );
      assert_false(
        unclickedCrossOrigin.hasBeenActive,
        "child-two hasBeenActive must be false"
      );
    }, "Check Initial states of user activation are all false");

    promise_test(async (t) => {
      await test_driver.click(soChild);
      const xoActivation = await requestXOReport();
      const soActivation = soChild.contentWindow.navigator.userActivation;
      assert_true(
        navigator.userActivation.isActive,
        "top-frame navigator.userActivation.isActive must be true"
      );
      assert_true(
        navigator.userActivation.hasBeenActive,
        "top-frame navigator.userActivation.hasBeenActive must be true"
      );
      assert_true(soActivation.isActive, "child-one isActive must be true");
      assert_true(
        soActivation.hasBeenActive,
        "child-one hasBeenActive must be true"
      );
      assert_false(xoActivation.isActive, "child-two isActive must be false");
      assert_false(
        xoActivation.hasBeenActive,
        "child-two hasBeenActive must be false"
      );
    }, "Check that activating a same-origin navigable doesn't activate a cross origin navigable");

    promise_test(async (t) => {
      await consumeTransientActivation();
      const soActivation = soChild.contentWindow.navigator.userActivation;
      // Before click...
      assert_false(
        navigator.userActivation.isActive,
        "top-frame navigator.userActivation.isActive must be false"
      );
      assert_true(
        navigator.userActivation.hasBeenActive,
        "top-frame navigator.userActivation.hasBeenActive must be true"
      );
      assert_false(soActivation.isActive, "child-one isActive must be false");
      assert_true(
        soActivation.hasBeenActive,
        "child-one hasBeenActive must be true"
      );
      const xoActivation = await requestXOReport();
      assert_false(xoActivation.isActive, "child-two isActive must be false");
      assert_false(
        xoActivation.hasBeenActive,
        "child-two hasBeenActive must be false"
      );

      // Click!
      const [, xoActivationAfterClick] = await Promise.all([
        test_driver.click(xoChild),
        receiveMessage("child-two-clicked"),
      ]);

      // After click...
      assert_true(
        navigator.userActivation.isActive,
        "top-frame navigator.userActivation.isActive must be true"
      );
      assert_true(
        navigator.userActivation.hasBeenActive,
        "top-frame navigator.userActivation.hasBeenActive must remain true"
      );
      assert_true(
        xoActivationAfterClick.isActive,
        "child-two isActive must be true"
      );
      assert_true(
        xoActivationAfterClick.hasBeenActive,
        "child-two hasBeenActive must be true"
      );
      assert_false(soActivation.isActive, "child-one isActive must be false");
      assert_true(
        soActivation.hasBeenActive,
        "child-one hasBeenActive must remain true"
      );
    }, "Clicking on the cross-origin navigable activates parent navigable.");
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
        "byte_end": 781,
        "byte_start": 773,
        "col": 3,
        "line": 24
      }
    },
    {
      "category": "Html",
      "code": "html.parser.nonspace_after_body",
      "message": "Non-space character after body.",
      "severity": "Error",
      "span": {
        "byte_end": 5221,
        "byte_start": 781,
        "col": 11,
        "line": 24
      }
    },
    {
      "category": "Html",
      "code": "html.parser.end_tag_after_body_closed",
      "message": "Saw an end tag after “body” had been closed.",
      "severity": "Error",
      "span": {
        "byte_end": 5230,
        "byte_start": 5221,
        "col": 3,
        "line": 150
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
  "source_name": "html/user-activation/propagation-same-and-cross-origin.sub.html"
}
```
