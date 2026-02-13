# html/interaction/focus/the-autofocus-attribute/no-cross-origin-autofocus.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/interaction/focus/the-autofocus-attribute/no-cross-origin-autofocus.html",
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
<script src="/common/get-host-info.sub.js"></script>
</head>
<body>
  <h1>Autofocus shouldn't work in cross-origin iframe.</h1>
  <iframe id="child" width="200" height="100"></iframe>

  <script>
    let parent_loaded = false;
    let child_loaded = false;

    async_test(function(t) {
      function pingChildIfBothFramesLoaded() {
        if (parent_loaded && child_loaded)
          frames[0].postMessage("report_focus_state", "*");
      }

      window.addEventListener("load", t.step_func(event => {
        parent_loaded = true;
        pingChildIfBothFramesLoaded();
      }));

      window.addEventListener("message", t.step_func(event => {
        if (event.data == "child_loaded") {
          child_loaded = true;
          pingChildIfBothFramesLoaded();
        } else if (event.data == "child_is_focused") {
          assert_unreached("The iframe shouldn't get focus");
        } else if (event.data == "child_is_not_focused") {
          t.done();
        }
      }));
      document.getElementById("child").src =
          get_host_info().HTTP_REMOTE_ORIGIN + "/html/interaction/focus/the-autofocus-attribute/resources/child-autofocus.html";
    }, "Autofocus shouldn't work in cross-origin iframe");
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
  "source_name": "html/interaction/focus/the-autofocus-attribute/no-cross-origin-autofocus.html"
}
```
