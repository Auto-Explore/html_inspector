# html/browsers/sandboxing/window-open-blank-from-different-initiator.html

Counts:
- errors: 1
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/sandboxing/window-open-blank-from-different-initiator.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<meta name="timeout" content="long">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/common/get-host-info.sub.js"></script>
<script src="/common/utils.js"></script>
<script>

// This is a regression test for https://crbug.com/1170038.
//
// A document creates a popup and makes it navigate elsewhere. The navigation
// will never commit. The popup has not completed any real load outside of the
// initial empty document. Then from a different window with a different CSP
// policy, make it navigate to about:blank.
//
// Web browser behavior might change depending on whether a pending navigation
// exists for in the popup or not. Both are tested here.

const same_origin = get_host_info().HTTP_ORIGIN;

// Return a promise, resolving when |element| triggers |event_name| event.
const future = (element, event_name) => {
  return new Promise(resolve => element.addEventListener(event_name, resolve));
};

// `createNewPopup` is a function returning a new window that has not committed
// any real load in its frame, outside of the initial empty document. The two
// tests below vary depending on whether there is a pending navigation in the
// frame or not.
const runTest = (description, createNewPopup) => {
  promise_test(async test => {
    // Open a same-origin window with a different CSP.
    const executor_path =
      "/html/browsers/sandboxing/resources/execute-postmessage.html?pipe=";
    const csp = "|header(Content-Security-Policy, " +
                "sandbox" +
                " allow-scripts" +
                " allow-popups" +
                " allow-same-origin" +
                " allow-popups-to-escape-sandbox";
    const executor = window.open(same_origin + executor_path + csp);
    const executor_reply = await future(window, "message");
    assert_equals(executor_reply.data, "ready");

    const popup_name = token();
    const popup = await createNewPopup(test, popup_name);

    // Request the first real load from a DIFFERENT window with different CSPs.
    const first_real_load = future(popup, "load");
    executor.postMessage(`window.open("about:blank", "${popup_name}");`);
    await first_real_load;

    // The new blank document in the popup must inherit CSPs from |executor|:
    let is_sandboxed = future(window, "message");
    popup.document.write(`
      <script>
        try {
          document.domain = document.domain;
          opener.opener.postMessage("not sandboxed", "*");
        } catch (error) {
          opener.opener.postMessage("sandboxed", "*");
        }
      </scr`+`ipt>
    `);
    assert_equals((await is_sandboxed).data, "sandboxed");
  }, description);
}

// Open a new window and start loading from an unresponsive server. The frame
// will be left with the initial empty document and a pending navigation.
runTest("One pending navigation", async (test, popup_name) => {
  const unresponsive_path = "/common/slow.py?delay=1000000";
  return window.open(same_origin + unresponsive_path, popup_name);
});

// Open a new window and start loading. The response is a 204 and the navigation
// is canceled. As a result, the frame will be left with the initial empty
// document and NO pending navigation.
runTest("No pending navigation", async (test, popup_name) => {
  const no_content_path = "/common/blank.html?pipe=status(204)"
  const popup = window.open(same_origin + no_content_path, popup_name);

  // Unfortunately, there are no web API to detect a navigation has been
  // canceled. Waiting using setTimeout is the only possible way to wait for it.
  await new Promise(r => test.step_timeout(r, 1000));

  return popup;
});

</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.parser.start_tag_without_doctype",
      "message": "Start tag seen without seeing a doctype first. Expected “<!DOCTYPE html>”.",
      "severity": "Error",
      "span": {
        "byte_end": 36,
        "byte_start": 0,
        "col": 1,
        "line": 1
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
  "source_name": "html/browsers/sandboxing/window-open-blank-from-different-initiator.html"
}
```
