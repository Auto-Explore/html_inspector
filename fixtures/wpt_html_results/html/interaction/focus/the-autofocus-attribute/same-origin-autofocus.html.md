# html/interaction/focus/the-autofocus-attribute/same-origin-autofocus.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/interaction/focus/the-autofocus-attribute/same-origin-autofocus.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<html>
<head>
<meta charset=utf-8>
<meta name="assert" content="`autofocus` should not work in the same origin iframe if there is a cross-origin iframe between the parent and the same origin iframe">
<title>autofocus in the same origin grand child iframe</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-vendor.js"></script>
<script src="/common/get-host-info.sub.js"></script>
<script src="resources/utils.js"></script>
</head>
<body>
  <iframe id="child" width="200" height="100"></iframe>
  <script>
    let parent_loaded = false;
    let grand_child_loaded = false;

    async_test(function(t) {
      async function pingChildIfBothFramesLoaded() {
        if (parent_loaded && grand_child_loaded) {
          await waitUntilStableAutofocusState();
          frames[0].postMessage("report_focus_state", "*");
        }
      }

      window.addEventListener("load", t.step_func(event => {
        parent_loaded = true;
        pingChildIfBothFramesLoaded();
      }));

      window.addEventListener("message", t.step_func(event => {
        if (event.data == "ready") {
          grand_child_loaded = true;
          pingChildIfBothFramesLoaded();
        } else if (event.data == "grand_child_is_focused") {
          assert_unreached("The grandchild iframe shouldn't get focus");
        } else if (event.data == "grand_child_is_not_focused") {
          t.done();
        }
      }));
      document.getElementById("child").src =
          get_host_info().HTTP_NOTSAMESITE_ORIGIN + "/html/interaction/focus/the-autofocus-attribute/resources/child-iframe.html";
    }, "Autofocus should not work in the same origin grand child iframe");
  </script>
</body>
</html>
```

```json
{
  "messages": [
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/interaction/focus/the-autofocus-attribute/same-origin-autofocus.html"
}
```
